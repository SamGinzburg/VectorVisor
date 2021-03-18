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
struct BatchResponse {
    requests: HashMap<u32, String>
}

impl BatchSubmitServer {
    pub fn start_server(sender: Sender<(Vec<u8>, usize)>, receiver: Receiver<(Vec<u8>, usize)>, vm_recv_condvar: Arc<Condvar>, num_vms: u32) -> JoinHandle<()> {
        let thandle = thread::spawn(move || {
            rouille::start_server("localhost:8000", move |request| {
                router!(request,
                    (GET) (/batch_submit/) => {
                        let json: BatchInput = try_or_400!(rouille::input::json_input(request));
                        println! ("json: {:?}", json);
                        dbg!(num_vms);
                        dbg!(json.requests.len());

                        for req in &json.requests {
                            // each request has an ID and a string (containing the json body)
                            let mut test = [0u8; 16384];

                            // copy the string to the buffer
                            let inc_req_as_bytes = req.req.as_bytes();
                            test[0..inc_req_as_bytes.len()].clone_from_slice(inc_req_as_bytes);

                            sender.send((test.to_vec(), inc_req_as_bytes.len())).unwrap();
                        }

                        dbg!("requests sent out!");
                        let mut responses: HashMap<u32, String> = HashMap::new();
                        // wait for the requests to complete
                        for _idx in 0..json.requests.len() {
                            dbg!("receiving!");
                            // each request has an ID and a string (containing the json body)
                            let (resp, len) = receiver.recv().unwrap();
                            // TODO: replace _idx with real req number
                            responses.insert(_idx.try_into().unwrap(), from_utf8(&resp[0..len]).unwrap().to_string());
                        }

                        dbg!("serving response!");
                        rouille::Response::json(&BatchResponse{requests: responses})
                    },
                    _ => rouille::Response::empty_404()
                )
            });
        });

        thandle
    }
}