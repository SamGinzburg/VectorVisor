use std::thread;
use std::thread::JoinHandle;
use std::sync::Condvar;
use std::sync::Arc;

use crossbeam::channel::Sender;
use crossbeam::channel::Receiver;
use crossbeam::channel::bounded;

use rouille::input::json;

use serde::Deserialize;

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

impl BatchSubmitServer {
    pub fn start_server(sender: Sender<u32>, receiver: Receiver<u32>, vm_recv_condvar: Arc<Condvar>, num_vms: u32) -> JoinHandle<()> {
        let thandle = thread::spawn(move || {
            rouille::start_server("localhost:8000", move |request| {
                router!(request,
                    (GET) (/batch_submit/) => {
                        let json: BatchInput = try_or_400!(rouille::input::json_input(request));
                        println! ("json: {:?}", json);
                        dbg!(num_vms);
                        // after receiving a batch of requests, send the requests to the waiting VMs
                        for _idx in 0..num_vms {
                            // each request has an ID and a string (containing the json body)
                            sender.send(0).unwrap();
                            vm_recv_condvar.notify_one();
                        }

                        dbg!("requests sent out!");

                        // wait for the requests to complete
                        for _idx in 0..num_vms {
                            dbg!("receiving!");
                            // each request has an ID and a string (containing the json body)
                            let resp = receiver.recv().unwrap();
                            println!("recv response: {:?}, idx: {:?}", resp, _idx);
                        }

                        dbg!("serving response!");
                        rouille::Response::text(format!("hello, {}", 1))
                    },
                    // The code block is called if none of the other blocks matches the request.
                    // We return an empty response with a 404 status code.
                    _ => rouille::Response::empty_404()
                )
            });
        });

        thandle
    }
}