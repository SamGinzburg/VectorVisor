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
	Text string `json:"text"`
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

// https://stackoverflow.com/questions/22892120/how-to-generate-a-random-string-of-a-fixed-length-in-go
var letterRunes = []rune("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")

func RandString(n int) string {
    b := make([]rune, n)
    for i := range b {
        b[i] = letterRunes[rand.Intn(len(letterRunes))]
    }
    return string(b)
}

func IssueRequests(ip string, port int, req_list []byte, batch_size int, num_batches_to_run int, data_ch chan<-[]byte) {
	final_request := bytes.NewReader(req_list)
	var DefaultClient = &http.Client{}
	addr := fmt.Sprintf("http://%s:%d/batch_submit/", ip, port)

	start := time.Now()
	http_request, _ := http.NewRequest("GET", addr, final_request)
	http_request.Header.Add("Content-Type", "application/json; charset=utf-8")
	//m := map[string]interface{}{}
	read_cnt := int64(0)
	for {
		//DefaultClient.Do(http_request)
		resp, _ := DefaultClient.Do(http_request)
		start_read := time.Now()
		body, _ := ioutil.ReadAll(resp.Body)
		read_secs := time.Since(start_read)
		read_cnt += read_secs.Nanoseconds()
		select {
			case data_ch <- body:
			default:
				break
		}
	}
	secs := time.Since(start)
	fmt.Printf("%.2f elapsed with response: %s, with RPS: %.2f\n", secs, addr, float64(batch_size) * float64(num_batches_to_run) / float64(secs.Seconds()))
	fmt.Printf("%.2f elapsed for reads\n", read_cnt)
  }

func main() {
	port, err := strconv.Atoi(os.Args[2])
	if err != nil {
		fmt.Println(err)
		os.Exit(2)
	}

	batch_size, err := strconv.Atoi(os.Args[3])
	if err != nil {
		fmt.Println(err)
		os.Exit(2)
	}

	num_vms, err := strconv.Atoi(os.Args[4])
	if err != nil {
		fmt.Println(err)
		os.Exit(2)
	}

	num_batches_to_run, err := strconv.Atoi(os.Args[5])
	if err != nil {
		fmt.Println(err)
		os.Exit(2)
	}

	reqs := make([]Message, batch_size)
	for i := 0; i < batch_size; i++ {
		p := payload{Text: RandString(1024 * 128)}
		msg, _ := json.Marshal(p)
		m := Message{Req_id: 0, Req: string(msg)}
		reqs[i] = m
	}
	request_body, _ := json.Marshal(MessageBatch{Requests: reqs})

	ch := make(chan []byte, num_vms*100000) // we prob won't exceed ~6.4M RPS ever
	benchmark_duration := 5 * time.Second
	bench_timer := time.NewTimer(benchmark_duration)
	for i := 0; i < num_vms; i++ {
		go IssueRequests(os.Args[1], port+i, request_body, batch_size, num_batches_to_run, ch)
	}

	<-bench_timer.C
	batches_completed := len(ch)
	fmt.Printf("Benchmark complete: %d batches completed\n", batches_completed)
	responses := make([][]byte, batches_completed)

	fmt.Printf("now waiting...\n")
	for i := 0; i < batches_completed; i++ {
		responses[i] = <-ch
	}

	duration := float64(benchmark_duration.Seconds())
	fmt.Printf("duration: %f\n", duration)
	// calculate the total RPS	
	total_rps := (float64(batch_size) * float64(num_vms) * float64(batches_completed)) / duration

	on_device_compute_time := 0.0
	device_queue_overhead := 0.0
	queue_submit_count := 0.0
	num_unique_fns_called := 0.0
	req_count := 0.0
	m := map[string]map[int]interface{}{}
	for i := 0; i < batches_completed; i++ {
		err := json.Unmarshal(responses[i], &m)
		if err != nil {
			fmt.Printf("Failed to unmarshal json error: %s, %s", err, string(responses[i]))
			os.Exit(2)
		}
		for key := range m["requests"] {
			m, err := m["requests"][key].(map[string]interface{})
			if !err {
				fmt.Printf("error: ", err)
				os.Exit(2)
			}
			on_device_compute_time += m["on_device_execution_time_ns"].(float64)
			device_queue_overhead += m["device_queue_overhead_time_ns"].(float64)
			queue_submit_count += m["queue_submit_count"].(float64)
			num_unique_fns_called += m["num_unique_fns_called"].(float64)
			req_count += 1
		}
	}

	on_device_compute_time = on_device_compute_time / req_count
	device_queue_overhead = device_queue_overhead / req_count
	queue_submit_count = queue_submit_count / req_count
	num_unique_fns_called = num_unique_fns_called / req_count

	fmt.Printf("Total RPS: %f\n", total_rps)
	fmt.Printf("Average on device compute time (ns): %f\n", on_device_compute_time)
	fmt.Printf("Average device queue overhead (ns): %f\n", device_queue_overhead)
	fmt.Printf("Average queue submit count: %f\n", queue_submit_count)
	fmt.Printf("Average num of unique fns called: %f\n", num_unique_fns_called)

	fmt.Printf("Parallel fraction of function (only applicable to GPU funcs): %f\n", (((on_device_compute_time+device_queue_overhead)/1000000000)*float64(num_batches_to_run)) / duration)
}
