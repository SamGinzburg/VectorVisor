use std::thread;
use std::thread::JoinHandle;
use std::sync::Condvar;
use std::sync::Arc;
use std::str::from_utf8;
use std::collections::HashMap;
use std::convert::TryInto;

use crossbeam::channel::Sender;
use crossbeam::channel::Receiver;
use crossbeam::channel::bounded;

use rouille::input::json;

use serde::Deserialize;
use serde::Serialize;

pub struct BatchSubmitServer {}

#[derive(Debug, Deserialize)]
struct FunctionInput {
    req: String,
    req_id: u32
}

#[derive(Debug, Deserialize)]
struct BatchInput {
    requests: Vec<FunctionInput>
}

#[derive(Debug, Serialize)]
struct BatchReply {
    response: String,
    on_device_execution_time_ns: u64,
    device_queue_overhead_time_ns: u64,
    queue_submit_count: u64,
    num_unique_fns_called: u64,
}

#[derive(Debug, Serialize)]
struct BatchResponse {
    requests: HashMap<u32, BatchReply>
}

impl BatchSubmitServer {
    pub fn start_server(sender: Sender<(Vec<u8>, usize)>, receiver: Receiver<(Vec<u8>, usize, u64, u64, u64, u64)>, num_vms: u32, server_ip: String, server_port: String) -> JoinHandle<()> {
        let thandle = thread::spawn(move || {
            rouille::start_server(format!("{}:{}", server_ip, server_port), move |request| {
                router!(request,
                    (GET) (/batch_submit/) => {
                        let json: BatchInput = try_or_400!(rouille::input::json_input(request));

                        for req in &json.requests {
                            // each request has an ID and a string (containing the json body)
                            let mut test = [0u8; 16384];

                            // copy the string to the buffer
                            let inc_req_as_bytes = req.req.as_bytes();
                            test[0..inc_req_as_bytes.len()].clone_from_slice(inc_req_as_bytes);
                            sender.send((test.to_vec(), inc_req_as_bytes.len())).unwrap();
                        }

                        let mut responses: HashMap<u32, BatchReply> = HashMap::new();

                        // wait for the requests to complete
                        for _idx in 0..json.requests.len() {
                            // each request has an ID and a string (containing the json body)
                            let (resp, len, on_dev_time, queue_submit_time, num_queue_submits, num_unique_fns) = receiver.recv().unwrap();
                            // TODO: replace _idx with real req number
                            responses.insert(_idx.try_into().unwrap(), BatchReply {
                                response: from_utf8(&resp[0..len]).unwrap().to_string(),
                                on_device_execution_time_ns: on_dev_time,
                                device_queue_overhead_time_ns: queue_submit_time,
                                queue_submit_count: num_queue_submits,
                                num_unique_fns_called: num_unique_fns,
                            });
                        }

                        rouille::Response::json(&BatchResponse{
                            requests: responses,
                        })
                    },
                    _ => rouille::Response::empty_404()
                )
            });
        });

        thandle
    }
}
