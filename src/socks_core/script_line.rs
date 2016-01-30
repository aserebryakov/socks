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


pub struct ScriptLine {
    number : u64,
    executable : bool,
    data : String,
    query : bool,
}


impl ScriptLine {
    pub fn new() -> ScriptLine {
        ScriptLine {
            number : 0,
            executable : false,
            data : String::new(),
            query : false,
        }
    }

    pub fn new_from_line(line : &String) -> ScriptLine {
        ScriptLine {
            number : 0,
            executable : false,
            data : line.clone(),
            query : false,
        }
    }

    pub fn clone(&self) -> ScriptLine {
        ScriptLine {
            number : self.number.clone(),
            executable : self.executable.clone(),
            data : self.data.clone(),
            query : self.query.clone(),
        }
    }

    pub fn number(&self) -> u64 {
        self.number
    }

    pub fn set_number(&mut self, n : &u64) {
        self.number = *n;
    }

    pub fn executable(&self) -> bool {
        self.executable
    }

    pub fn set_executable(&mut self, e : &bool) {
        self.executable = *e;
    }

    pub fn data(&self) -> &String {
        &self.data
    }

    pub fn set_data(&mut self, d : &String) {
        self.data = d.clone();
    }

    pub fn query(&self) -> bool {
        self.query
    }

    pub fn set_query(&mut self, q : &bool) {
        self.query = q.clone();
    }

    pub fn dbg_print(&self) {
        println!("number = {}", self.number);
        println!("executable = {}", self.executable);
        println!("data = \"{}\"", self.data);
        println!("query = {}", self.query);
    }
}
