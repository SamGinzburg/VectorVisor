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

func generatePdf()  []byte {
    pdf := gopdf.GoPdf{}

    pdf.Start(gopdf.Config{ PageSize: *gopdf.PageSizeA4 })
    pdf.AddPage()

    fmt.Printf("test\n")
    err := pdf.AddTTFFontDataWithOption("times", times, defaultTtfFontOption())
    if err != nil {
        log.Print(err.Error())
        panic (err)
    }
    fmt.Printf("loaded fonts\n")

    err = pdf.SetFont("times", "", 14)
    if err != nil {
        log.Print(err.Error())
        panic (err)
    }

    fmt.Printf("set fonts\n")
    pdf.SetXY(30, 70)
    pdf.Text("Hello world!")
    pdf.SetXY(30, 100)
    pdf.Text("Hello world again!")
    pdf.SetXY(30, 100)
    pdf.Text("A third time!")

    return pdf.GetBytesPdf()
}

func main() {
    for {
        runtime.InitHeap()
        input_buf := make([]byte, 1024 * 512)
        in_size := C.serverless_invoke((*C.char)(unsafe.Pointer(&input_buf[0])), 1024 * 512)
        println(in_size)
        fmt.Printf("%v\n", string(input_buf[:in_size]))

        value, dtype, _, _ := jsonparser.Get(input_buf[:in_size], "inputs")
        fmt.Printf("%v\t%v\n", value, dtype)

        jsonparser.ArrayEach(value, func(value []byte, dataType jsonparser.ValueType, offset int, err error) {
            name, _, _, _ := jsonparser.Get(value, "name")
            purchases_arr, _, _, _ := jsonparser.Get(value, "purchases")
            prices_arr, _, _, _ := jsonparser.Get(value, "price")

            fmt.Println(string(name))
            jsonparser.ArrayEach(purchases_arr, func(value []byte, dataType jsonparser.ValueType, offset int, err error) {
                fmt.Println(string(value))
            })
            jsonparser.ArrayEach(prices_arr, func(value []byte, dataType jsonparser.ValueType, offset int, err error) {
                fmt.Println(string(value))
            })

        })

        /*
        pdf_reqs := map[string]interface{}{}
        err := json.Unmarshal(input_buf[:in_size], &pdf_reqs)
        if err != nil {
            panic(err)
        }
        fmt.Printf("%v\n", pdf_reqs)
        */

        result := generatePdf()

        println(len(result))

        C.serverless_response((*C.char)(unsafe.Pointer(&input_buf[0])), 1024 * 512)
    }
}
