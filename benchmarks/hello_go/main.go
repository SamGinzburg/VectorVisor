package main

// #include "serverless.c"
import "C"

import (
    "fmt"
    "runtime"
    //"unsafe"
    "log"
    _ "embed"
    "github.com/signintech/gopdf"
)

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
    pdf.SetNoCompression()
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

    pdf.SetNoCompression()
    return pdf.GetBytesPdf()
}

func main() {

    //input_buf := make([]byte, 1024 * 512)

    for {
        runtime.GC()
        //C.serverless_invoke((*C.char)(unsafe.Pointer(&input_buf)), 1024 * 512)
        result := generatePdf()
        //copy(input_buf, result)
        println(len(result))


        runtime.GC()
        //C.serverless_response((*C.char)(unsafe.Pointer(&input_buf)), 1024 * 512)
    }
    //fmt.Printf("%v\n", len(result))
}
