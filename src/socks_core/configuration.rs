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


pub struct Configuration {
    non_executable_criterias : Vec<String>,
    query_criterias: Vec<String>,
    host : String,
    stop_byte : u8,
}


impl Configuration {
    pub fn new(non_exec: &Vec<String>,
               query: &Vec<String>,
               h: &String,
               stop: &u8)
               -> Configuration {
        Configuration {
            non_executable_criterias : non_exec.clone(),
            query_criterias : query.clone(),
            host : h.clone(),
            stop_byte : stop.clone(),
        }
    }

    pub fn new_empty() -> Configuration {
        Configuration {
            non_executable_criterias : Vec::<String>::new(),
            query_criterias: Vec::<String>::new(),
            host : String::new(),
            stop_byte : 0,
        }
    }

    pub fn clone(&self) -> Configuration {
        Configuration {
            non_executable_criterias : self.non_executable_criterias.clone(),
            query_criterias : self.query_criterias.clone(),
            host : self.host.clone(),
            stop_byte : self.stop_byte.clone(),
        }
    }

    pub fn non_executable_criterias(&self) -> &Vec<String> {
        &self.non_executable_criterias
    }

    pub fn query_criterias(&self) -> &Vec<String> {
        &self.query_criterias
    }

    pub fn host(&self) -> &String {
        &self.host
    }

    pub fn stop_byte(&self) -> &u8 {
        &self.stop_byte
    }

    pub fn write(&self) {
        println!("{:?}", self.non_executable_criterias);
        println!("{:?}", self.query_criterias);
        println!("{}", self.host);
        println!("{}", self.stop_byte);
    }
}
