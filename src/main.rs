mod opencl_writer;

use std::fs;
use wast::parser::{ParseBuffer};

fn main() {

    //let file = fs::read_to_string("examples/arithmetic/lt.wat");
    let file = fs::read_to_string("examples/branches/loop.wat");
    //let file = fs::read_to_string("examples/wasi_examples/fd_write.wat");
    
    let filedata = match file {
        Ok(text) => text,
        Err(e) => panic!(e),
    };

    let pb = ParseBuffer::new(&filedata).unwrap();

    let mut ast = opencl_writer::OpenCLCWriter::new(&pb);
    let result = ast.parse_file();

    println!("{:?}", result);
    match result {
        Ok(_) => ast.write_opencl_file("test.c", false),
        Err(_) => println!("Unable to parse wat file"),
    }

    //println!("{:?}", ast);
}
