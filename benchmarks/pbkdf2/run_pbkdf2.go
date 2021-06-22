package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"net/http"
	"time"
	"encoding/json"
	"bytes"
	"strconv"
	"math/rand"
)

type payload struct {
	Text string `json:"password"`
}

type Message struct {
	Req_id int `json:"req_id"`
	Req string `json:"req"`
}

type MessageBatch struct {
	Requests []Message `json:"requests"`
}

type VmmResponse struct {
	response string
	on_device_execution_time_ns float64
	device_queue_overhead_time_ns float64
	queue_submit_count float64
	num_unique_fns_called float64
}

var NUM_PARAMS = 256;

var client = &http.Client{}

func RandIntSlice(n int) []string {
    b := make([]string, n)
    for i := range b {
        b[i] = RandString(rand.Intn(10)) //rand.Float64() * (10000)
    }
    return b
}

// https://stackoverflow.com/questions/22892120/how-to-generate-a-random-string-of-a-fixed-length-in-go
var letterRunes = []rune("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")

func RandString(n int) string {
    b := make([]rune, n)
    for i := range b {
        b[i] = letterRunes[rand.Intn(len(letterRunes))]
    }
    return string(b)
}

func IssueRequests(ip string, port int, req [][]byte, data_ch chan<-[]byte, end_chan chan bool) {
	addr := fmt.Sprintf("http://%s:%d/batch_submit/", ip, port)
	http_request, _ := http.NewRequest("GET", addr, nil)
	http_request.Header.Add("Content-Type", "application/json; charset=utf-8")

	for {
		http_request.Body = ioutil.NopCloser(bytes.NewReader(req[rand.Intn(NUM_PARAMS)]))
		start_read := time.Now()
		_ = start_read
		resp, err := client.Do(http_request)
		if err != nil {
			fmt.Printf("client err: %s\n", err)
			// check to see if we are done
			if len(end_chan) > 0 {
				return;
			}
			continue
		}

		body, err := ioutil.ReadAll(resp.Body)
		//fmt.Printf("%s\n", body)
		resp.Body.Close()
		read_secs := time.Since(start_read)
		_ = read_secs
		if err != nil {
			//fmt.Printf("err: %s\n", err)
		}

		// if we get a hangup from the server, continue
		if resp.StatusCode != http.StatusOK {
			continue
		} else {
			//fmt.Printf("E2E req time: %s\n", read_secs)
		}

		select {
			case data_ch <- body:
			default:
				return;
		}

		// check to see if we are done
		if len(end_chan) > 0 {
			return;
		}
	}
}

func main() {
	port, err := strconv.Atoi(os.Args[2])
	if err != nil {
		fmt.Println(err)
		os.Exit(2)
	}

	num_vms, err := strconv.Atoi(os.Args[3])
	if err != nil {
		fmt.Println(err)
		os.Exit(2)
	}

	num_vmgroups, err := strconv.Atoi(os.Args[4])
	if err != nil {
		fmt.Println(err)
		os.Exit(2)
	}

	timeout_secs, err := strconv.Atoi(os.Args[5])
	if err != nil {
		fmt.Println(err)
		os.Exit(2)
	}


	reqs := make([][]byte, NUM_PARAMS)
	for i := 0; i < NUM_PARAMS; i++ {
		p := payload{Text: RandString(16)}
		request_body, _ := json.Marshal(p)
		reqs[i] = request_body
	}

	tr := &http.Transport{
		MaxIdleConnsPerHost: num_vms * 2 * num_vmgroups,
	}
	client = &http.Client{Transport: tr}


	ch := make(chan []byte, num_vms*100000) // we prob won't exceed ~6.4M RPS ever
	termination_chan := make(chan bool, num_vms)

	benchmark_duration := time.Duration(timeout_secs) * time.Second
	bench_timer := time.NewTimer(benchmark_duration)
	for vmgroup := 0 ; vmgroup < num_vmgroups; vmgroup++ {
		for i := 0; i < num_vms; i++ {
			go IssueRequests(os.Args[1], port+vmgroup, reqs, ch, termination_chan)
		}
	}

	<-bench_timer.C
	batches_completed := len(ch)
	fmt.Printf("Benchmark complete: %d batches completed\n", batches_completed)
	responses := make([][]byte, batches_completed)
	for i := 0; i < batches_completed; i++ {
		responses[i] = <-ch
	}

	duration := float64(benchmark_duration.Seconds())

	fmt.Printf("duration: %f\n", duration)

	for i := 0; i < num_vms; i++ {
		termination_chan <- true
	}

	// calculate the total RPS	
	total_rps := (float64(batches_completed)) / duration
	fmt.Printf("Total RPS: %f\n", total_rps)

	on_device_compute_time := 0.0
	device_queue_overhead := 0.0
	queue_submit_count := 0.0
	num_unique_fns_called := 0.0
	req_count := 0.0
	m := map[string]interface{}{}
	for i := 0; i < batches_completed; i++ {
		err := json.Unmarshal(responses[i], &m)
		if err != nil {
			fmt.Printf("Failed to unmarshal json error: %s, %s", err, string(responses[i]))
			os.Exit(2)
		}
		on_device_compute_time += m["on_device_execution_time_ns"].(float64)
		device_queue_overhead += m["device_queue_overhead_time_ns"].(float64)
		queue_submit_count += m["queue_submit_count"].(float64)
		num_unique_fns_called += m["num_unique_fns_called"].(float64)
		req_count += 1
	}

	on_device_compute_time = on_device_compute_time / req_count
	device_queue_overhead = device_queue_overhead / req_count
	queue_submit_count = queue_submit_count / req_count
	num_unique_fns_called = num_unique_fns_called / req_count

	fmt.Printf("Average on device compute time (ns): %f\n", on_device_compute_time)
	fmt.Printf("Average device queue overhead (ns): %f\n", device_queue_overhead)
	fmt.Printf("Average queue submit count: %f\n", queue_submit_count)
	fmt.Printf("Average num of unique fns called: %f\n", num_unique_fns_called)

	fmt.Printf("Parallel fraction of function (only applicable to GPU funcs): %f\n", (((on_device_compute_time+device_queue_overhead)/1000000000)) / duration)
}
