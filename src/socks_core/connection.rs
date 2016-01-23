//
//   Copyright 2016 Alexander Serebryakov
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.
//


use std::io::prelude::*;
use std::net::TcpStream;


pub struct Connection {
    stream : TcpStream,
}


impl Connection {
    pub fn new(addr : &str) -> Connection {
        let s = TcpStream::connect(addr).unwrap();

        Connection {
            stream : s
        }
    }

    pub fn send(&mut self, data : &Vec::<u8>, query : bool) -> Vec<u8> {
        let mut raw_bytes = Vec::<u8>::new();

        if cmd.executable() {
            self.stream.write(data);

            if cmd.query() {
                self.stream.read_to_end(&mut raw_bytes);
            }
        }

        raw_bytes
    }
}
