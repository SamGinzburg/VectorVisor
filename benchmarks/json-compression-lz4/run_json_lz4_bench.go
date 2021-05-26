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
)

type payload struct {
	Text string
}

type Message struct {
    Req_id int
	Req string
}

type MessageBatch struct {
	Requests []Message
}

func IssueRequests(ip string, port string, req_list []byte, ch chan<-string) {
	final_request := bytes.NewReader(req_list)
	addr := fmt.Sprintf("http://%s:%s/batch_submit/", ip, port)
	start := time.Now()
	resp, _ := http.NewRequest("GET", addr, final_request)
	secs := time.Since(start).Seconds()
	body, _ := ioutil.ReadAll(resp.Body)
	ch <- fmt.Sprintf("%.2f elapsed with response: %s %s", secs, body, addr)
  }

func main() {
	// generate requests
	batch_size, err := strconv.Atoi(os.Args[3])
    if err != nil {
        // handle error
        fmt.Println(err)
        os.Exit(2)
    }

	num_vms, err := strconv.Atoi(os.Args[4])
    if err != nil {
        // handle error
        fmt.Println(err)
        os.Exit(2)
    }

	reqs := make([]Message, batch_size)
	for i := 0; i < batch_size; i++ {
		temp_payload := payload{Text: "test"}
		payload, _ := json.Marshal(temp_payload)
		m := Message{Req_id: 0, Req: string(payload)}
		reqs[i] = m
	}
	request_body, _ := json.Marshal(MessageBatch{Requests: reqs})

	ch := make(chan string)
	start := time.Now()
	for i := 0; i < num_vms; i++ {
		go IssueRequests(os.Args[1], os.Args[2], request_body, ch)
	}

	fmt.Println(<-ch)
	fmt.Printf("%.2fs elapsed\n", time.Since(start).Seconds())
  }