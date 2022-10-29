package main

import (
	"bytes"
	b64 "encoding/base64"
	"encoding/json"
	"fmt"
	"io"
	"io/ioutil"
	"math/rand"
	"net/http"
	"os"
	"strconv"
	"time"
)

type payload struct {
	Text string `json:"encoded_str"`
}

type Message struct {
	Req_id int    `json:"req_id"`
	Req    string `json:"req"`
}

type MessageBatch struct {
	Requests []Message `json:"requests"`
}

type VmmResponse struct {
	response                      string
	on_device_execution_time_ns   float64
	device_queue_overhead_time_ns float64
	queue_submit_count            float64
	num_unique_fns_called         float64
}

var NUM_PARAMS = 256

var client = &http.Client{}

func RandIntSlice(n int) []int {
	b := make([]int, n)
	for i := range b {
		b[i] = rand.Intn(10000)
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
	return b64.StdEncoding.EncodeToString([]byte(string(b)))
}

func RandMessage(n int) string {
	// no more than 1MB
	file_data, err := ioutil.ReadFile(fmt.Sprintf("messages/%d.txt", n))
	_ = file_data
	if err != nil {
		panic(err)
	}
	return b64.StdEncoding.EncodeToString([]byte(file_data))
}

func IssueRequests(ip string, port int, req [][]byte, exec_time chan<- float64, latency chan<- float64, queue_time chan<- float64, submit_count chan<- float64, unique_fns chan<- float64, request_queue_time chan<- float64, device_time_ch chan<- float64, overhead_ch chan<- float64, compile_ch chan<- float64, end_chan chan bool) {
	addr := fmt.Sprintf("http://%s:%d/batch_submit/", ip, port)
	http_request, _ := http.NewRequest("GET", addr, nil)
	http_request.Header.Add("Content-Type", "application/json; charset=utf-8")

	on_device_compute_time := 0.0
	device_queue_overhead := 0.0
	queue_submit_count := 0.0
	num_unique_fns_called := 0.0
	req_queue_time := 0.0
	device_time := 0.0
	overhead_time := 0.0
	compile_time := 0.0

	for {
		http_request.Body = ioutil.NopCloser(bytes.NewReader(req[rand.Intn(NUM_PARAMS)]))
		start_read := time.Now()
		_ = start_read
		resp, err := client.Do(http_request)
		if err != nil {
			// check to see if we are done
			if len(end_chan) > 0 {
				return
			}
			continue
		}
		/*
			body, err := ioutil.ReadAll(resp.Body)
			if err != nil {
				panic(err)
			}
			_ = body
		*/
		io.Copy(ioutil.Discard, resp.Body)
		resp.Body.Close()
		read_secs := time.Since(start_read)
		_ = read_secs

		// if we get a hangup from the server, continue
		if resp.StatusCode != http.StatusOK {
			continue
		} else {
			//fmt.Printf("E2E req time: %s\n", read_secs)
			//fmt.Printf("%s\n", body)
		}

		on_device_compute_time, _ = strconv.ParseFloat(resp.Header.Get("on_device_time"), 64)
		device_queue_overhead, _ = strconv.ParseFloat(resp.Header.Get("queue_submit_time"), 64)
		queue_submit_count, _ = strconv.ParseFloat(resp.Header.Get("num_queue_submits"), 64)
		num_unique_fns_called, _ = strconv.ParseFloat(resp.Header.Get("num_unique_fns"), 64)
		req_queue_time, _ = strconv.ParseFloat(resp.Header.Get("req_queue_time"), 64)
		device_time, _ = strconv.ParseFloat(resp.Header.Get("device_time"), 64)
		overhead_time, _ = strconv.ParseFloat(resp.Header.Get("overhead_time_ns"), 64)
		compile_time, _ = strconv.ParseFloat(resp.Header.Get("compile_time_ns"), 64)
		select {
		case compile_ch <- compile_time:
		default:
			return
		}
		select {
		case overhead_ch <- overhead_time:
		default:
			return
		}
		select {
		case device_time_ch <- device_time:
		default:
			return
		}
		select {
		case request_queue_time <- req_queue_time:
		default:
			return
		}
		select {
		case exec_time <- on_device_compute_time:
		default:
			return
		}
		select {
		case latency <- float64(read_secs):
		default:
			return
		}
		select {
		case queue_time <- device_queue_overhead:
		default:
			return
		}
		select {
		case submit_count <- queue_submit_count:
		default:
			return
		}
		select {
		case unique_fns <- num_unique_fns_called:
		default:
			return
		}
		// check to see if we are done
		if len(end_chan) > 0 {
			return
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

	input_size, err := strconv.Atoi(os.Args[6])
	if err != nil {
		fmt.Println(err)
		os.Exit(2)
	}

	reqs := make([][]byte, NUM_PARAMS)
	for i := 0; i < NUM_PARAMS; i++ {
		p := payload{Text: RandMessage(rand.Intn(input_size))}
		request_body, _ := json.Marshal(p)
		reqs[i] = request_body
	}

	tr := &http.Transport{
		MaxIdleConnsPerHost: num_vms * 2 * num_vmgroups,
	}
	client = &http.Client{Transport: tr}

	addr := fmt.Sprintf("http://%s:%d/is_active/", os.Args[1], port)
	http_request, _ := http.NewRequest("GET", addr, nil)
	http_request.Header.Add("Content-Type", "application/json; charset=utf-8")
	for {
		resp, err := client.Do(http_request)
		if err != nil {
			//fmt.Printf("is_active route not running yet...\n")
			time.Sleep(2000 * time.Millisecond)
			continue
		}
		if resp.StatusCode != http.StatusOK {
			//fmt.Printf("is_active route not running yet...\n")
			time.Sleep(2000 * time.Millisecond)
			continue
		} else {
			break
		}
	}
	//fmt.Printf("server is active... starting benchmark\n")
	time.Sleep(5000 * time.Millisecond)

	ch_exec_time := make(chan float64, 1000000)
	ch_latency := make(chan float64, 1000000)
	ch_queue_time := make(chan float64, 1000000)
	ch_submit := make(chan float64, 1000000)
	ch_unique_fns := make(chan float64, 1000000)
	ch_req_queue_time := make(chan float64, 1000000)
	ch_device_time := make(chan float64, 1000000)
	ch_overhead := make(chan float64, 1000000)
	ch_compile := make(chan float64, 1000000)

	termination_chan := make(chan bool, num_vms)

	benchmark_duration := time.Duration(timeout_secs) * time.Second
	bench_timer := time.NewTimer(benchmark_duration)
	for vmgroup := 0; vmgroup < num_vmgroups; vmgroup++ {
		for i := 0; i < num_vms; i++ {
			go IssueRequests(os.Args[1], port+vmgroup, reqs, ch_exec_time, ch_latency, ch_queue_time, ch_submit, ch_unique_fns, ch_req_queue_time, ch_device_time, ch_overhead, ch_compile, termination_chan)
		}
	}

	<-bench_timer.C

	for i := 0; i < num_vms; i++ {
		termination_chan <- true
	}

	batches_completed := len(ch_exec_time)
	fmt.Printf("Benchmark complete: %d requests completed\n", batches_completed)
	exec_time := 0.0
	latency := 0.0
	queue_time := 0.0
	submit_count := 0.0
	unique_fns := 0.0
	req_queue_time := 0.0
	device_time := 0.0
	overhead := 0.0
	compile := 0.0

	for i := 0; i < batches_completed; i++ {
		exec_time += <-ch_exec_time
		latency += <-ch_latency
		queue_time += <-ch_queue_time
		submit_count += <-ch_submit
		unique_fns += <-ch_unique_fns
		req_queue_time += <-ch_req_queue_time
		device_time += <-ch_device_time
		overhead += <-ch_overhead
		compile += <-ch_compile
	}

	duration := float64(benchmark_duration.Seconds())
	exec_time = exec_time / float64(batches_completed)
	latency = latency / float64(batches_completed)
	queue_time = queue_time / float64(batches_completed)
	submit_count = submit_count / float64(batches_completed)
	unique_fns = unique_fns / float64(batches_completed)
	req_queue_time = req_queue_time / float64(batches_completed)
	device_time = device_time / float64(batches_completed)
	overhead = overhead / float64(batches_completed)
	compile = compile / float64(batches_completed)

	fmt.Printf("duration: %f\n", duration)
	// calculate the total RPS
	total_rps := (float64(batches_completed)) / duration
	fmt.Printf("Total RPS: %f\n", total_rps)
	fmt.Printf("On device execution time: %f\n", exec_time)
	fmt.Printf("Average request latency: %f\n", latency)
	fmt.Printf("queue submit time: %f\n", queue_time)
	fmt.Printf("submit count: %f\n", submit_count)
	fmt.Printf("unique fns: %f\n", unique_fns)
	fmt.Printf("Request Queue Time: %f\n", req_queue_time)
	fmt.Printf("Device Time: %f\n", device_time)
	fmt.Printf("overhead: %f\n", overhead)
	fmt.Printf("compile time: %f\n", compile)
}
