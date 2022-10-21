package main
import (
    //"fmt"
    //"log"
    "bytes"
    _ "embed"
    "github.com/signintech/gopdf"
)

//go:embed LiberationSerif-Regular.ttf
var libsanserif []byte

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

func main() {
    //fmt.Printf("%v\n", libsanserif)

    pdf := gopdf.GoPdf{}
    pdf.Start(gopdf.Config{ PageSize: *gopdf.PageSizeA4 })
    pdf.AddPage()


    err := pdf.AddTTFFontByReaderWithOption("libserif", bytes.NewReader(libsanserif), defaultTtfFontOption())
    //err := pdf.AddTTFFont("libserif", "LiberationSerif-Regular.ttf")
    if err != nil {
        //log.Print(err.Error())
        return
    }

    //fmt.Printf("loaded fonts\n")

    err2 := pdf.SetFont("libserif", "", 14)
    if err2 != nil {
        //log.Print(err.Error())
        return
    }

    //fmt.Printf("set fonts\n")
    pdf.SetXY(30, 70)
    pdf.Text("Hello world!")
    pdf.SetXY(30, 100)
    pdf.Text("Hello world again!")
    pdf.SetXY(30, 100)
    pdf.Text("A third time!")

    //fmt.Printf("%v\n", pdf.GetBytesPdf())
    println(pdf.GetBytesPdf())
}
