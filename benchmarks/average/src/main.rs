use wasm_serverless_invoke::*;
use wasm_serverless_invoke::wasm_handler::WasmHandler;
use wasm_serverless_invoke::wasm_handler::SerializationFormat::{MsgPack, Json};
use serde::Deserialize;
use serde::Serialize;
use hdrhistogram::Histogram;

#[derive(Debug, Deserialize)]
struct FuncInput {
    numbers: Vec<u64>
}

#[derive(Debug, Serialize)]
struct FuncResponse {
    min: u64,
    max: u64,
    mean: f64,
    stddev: f64,
    p50: u64,
    p99: u64,
    p999: u64,
}

// Take in a list of numbers and compute the average
fn histogram(event: FuncInput) -> FuncResponse {
    let mut histogram = Histogram::<u64>::new(2).unwrap();

    for item in &event.numbers {
        histogram.record(*item);
    }

    FuncResponse { 
        min: histogram.min(),
        max: histogram.max(),
        mean: histogram.mean(),
        stddev: histogram.stdev(),
        p50: histogram.value_at_percentile(50.0),
        p99: histogram.value_at_percentile(99.0),
        p999: histogram.value_at_percentile(99.9),
    }
}

fn main() {
    let handler = WasmHandler::new(&histogram);
    handler.run_with_format(1024*1024, MsgPack);
}
