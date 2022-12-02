package main;

// #include "serverless.c"
import "C"

import (
    "github.com/json-iterator/tinygo"
    //"encoding/json"
    "unsafe"
    "strings"
    //"runtime"
)


//go:generate go run github.com/json-iterator/tinygo/gen
type Payload struct {
	Tweets  []string `json:"tweets"`
}

//go:generate go run github.com/json-iterator/tinygo/gen
type Response struct {
	Tokenized  [][]string
    Hashtags   [][]string
}

func Map[T, U any](ts []T, f func(T) U) []U {
    us := make([]U, len(ts))
    for i := range ts {
        us[i] = f(ts[i])
    }
    return us
}

func Filter(vs []string, f func(string) bool) []string {
    filtered := make([]string, 0)
    for _, v := range vs {
        if f(v) {
            filtered = append(filtered, v)
        }
    }
    return filtered
}

func main() {

    // Patches to get json encoding working with tinygo
    // list all the types you need to unmarshal here
    json := jsoniter.CreateJsonAdapter(Payload_json{}, Response_json{}) 

    // Use this as a set, track all stopwords
    stopwordSet := make(map[string]bool)
    for _, word := range stopWords {
        stopwordSet[word] = true
    }

    input_buf := make([]byte, 1024*512)

	for {
		in_size := C.serverless_invoke((*C.char)(unsafe.Pointer(&input_buf[0])), 1024*512)
        // if in_size == 0, there is no input
        if in_size == 0 {
            fakeaddr := uintptr(0x0)
		    C.serverless_response((*C.char)(unsafe.Pointer(fakeaddr)), 0)
            continue
        }
        var input Payload;
        json.Unmarshal(input_buf[0:in_size], &input); 
    
        C.vectorvisor_barrier();

        // First tokenize each tweet []string --> [][]string
        var tokenized = make([][]string, 0)
        for _, e := range input.Tweets {
            tokenized = append(tokenized, strings.Split(e, " "))
        }

        C.vectorvisor_barrier();

        // Now process each tweet, extracting stop words 
        tokenized = Map(tokenized, func(tweet []string) []string {
            return Filter(tweet, func(word string) bool {
                if _, ok := stopwordSet[word]; ok {
                    return false
                }
                return true
            })
        })

        C.vectorvisor_barrier();

        // Get the hashtags
        var tags = make([][]string, 0)
        for _, tweet := range tokenized {
            var tweetTags = make([]string, 0)
            for _, word := range tweet {
                if strings.HasPrefix(word, "#") {
                    tweetTags = append(tweetTags, word)
                }
            }
            tags = append(tags, tweetTags)
        }

        C.vectorvisor_barrier();

        var response Response;
        response.Tokenized = tokenized;
        response.Hashtags = tags;
        bytes, _ := json.Marshal(response);

	    C.serverless_response((*C.char)(unsafe.Pointer(&bytes[0])), (C.uint)(len(bytes)))
	}
}

