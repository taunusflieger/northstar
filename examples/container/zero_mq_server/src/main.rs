// Copyright (c) 2020 ESRLabs
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.


use std::thread;
use std::time::Duration;

fn main() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("ipc:///sockets/test.soc").is_ok());

    let mut msg = zmq::Message::new();
    loop {
        responder.recv(&mut msg, 0).unwrap();
        println!("Received {}", msg.as_str().unwrap());
        thread::sleep(Duration::from_millis(1000));
        responder.send("World", 0).unwrap();
    }
}