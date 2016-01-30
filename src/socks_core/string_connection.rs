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


use socks_core::connection::Connection;


pub struct StringConnection {
    connection : Connection
}


impl StringConnection {
    pub fn new(addr : &str) -> StringConnection {
        StringConnection {
            connection : Connection::new(addr),
        }
    }

    pub fn send(&mut self, data : &String) {
        self.connection.send(&data.as_bytes().to_vec());
    }

    pub fn receive(&mut self, data : &mut String) {
        let mut raw_data = Vec::<u8>::new();
        self.connection.receive(&mut raw_data);
    }
}
