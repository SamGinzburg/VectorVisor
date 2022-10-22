package main

// #include "serverless.c"
import "C"

import (
    "fmt"
    "runtime"
    "unsafe"
    "log"
    _ "embed"
    "github.com/signintech/gopdf"
    "github.com/buger/jsonparser"
    //"encoding/json"
)

//tinyjson:json
type Payload struct {
    Text string
    Purchases []string
    Price []float64
}

//tinyjson:json
type BatchPayload struct {
    Inputs []Payload
}

//go:embed times.ttf
var times []byte

//Regular - font style regular
//const Regular = 0 //000000
//Italic - font style italic
//const Italic = 1 //000001
//Bold - font style bold
//const Bold = 2 //000010
//Underline - font style underline
//const Underline = 4 //000100

func DefaultOnGlyphNotFoundSubstitute(r rune) rune {
    return rune('\u0020')
}

func defaultTtfFontOption() gopdf.TtfOption {
    var defa gopdf.TtfOption
    defa.UseKerning = false
    defa.Style = 0
    defa.OnGlyphNotFoundSubstitute = DefaultOnGlyphNotFoundSubstitute
    return defa
}

func generatePdf(name string, purchases []string, prices []string)  []byte {
    pdf := gopdf.GoPdf{}

    pdf.Start(gopdf.Config{ PageSize: *gopdf.PageSizeA4 })
    pdf.AddPage()

    err := pdf.AddTTFFontDataWithOption("times", times, defaultTtfFontOption())
    if err != nil {
        log.Print(err.Error())
        panic (err)
    }

    err = pdf.SetFont("times", "", 14)
    if err != nil {
        log.Print(err.Error())
        panic (err)
    }

    // Header
    pdf.SetXY(30, 50)
    pdf.Text("Fake bill for:\t" + name)

    start := 100
    iter := zip(purchases, prices)
    for tuple := iter(); tuple != nil; tuple = iter() {
        pdf.SetXY(30, start)
        pdf.Text(tuple[0] + "\t$" + tuple[1])
        start += 30
    }

    return pdf.GetBytesPdf()
}

func main() {
    for {
        runtime.InitHeap()
        input_buf := make([]byte, 1024 * 512)
        in_size := C.serverless_invoke((*C.char)(unsafe.Pointer(&input_buf[0])), 1024 * 512)
        //println(in_size)
        //fmt.Printf("%v\n", string(input_buf[:in_size]))

        value, _, _, _ := jsonparser.Get(input_buf[:in_size], "inputs")
        purchases := make([]string, 0)
        prices := make([]string, 0)
        bill_name := ""
        jsonparser.ArrayEach(value, func(value []byte, dataType jsonparser.ValueType, offset int, err error) {
            name, _, _, _ := jsonparser.Get(value, "name")
            purchases_arr, _, _, _ := jsonparser.Get(value, "purchases")
            prices_arr, _, _, _ := jsonparser.Get(value, "price")
            bill_name = string(name)
            jsonparser.ArrayEach(purchases_arr, func(value []byte, dataType jsonparser.ValueType, offset int, err error) {
                purchases = append(purchases, string(value))
            })
            jsonparser.ArrayEach(prices_arr, func(value []byte, dataType jsonparser.ValueType, offset int, err error) {
                prices = append(prices, string(value))
            })

        })

        fmt.Println(bill_name)
        fmt.Println(prices)
        fmt.Println(purchases)

        result := generatePdf()

        println(len(result))

        C.serverless_response((*C.char)(unsafe.Pointer(&input_buf[0])), 1024 * 512)
    }
}