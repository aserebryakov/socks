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


use socks_core::script_line::ScriptLine;
use socks_core::configuration::Configuration;
use socks_core::string_connection::StringConnection;


pub struct Executer {
    configuration : Configuration,
    script : Vec<ScriptLine>,
    pc : u64,
    connection : StringConnection,
}


impl Executer {
    pub fn new(c : &Configuration, s : Vec<ScriptLine>) -> Executer {
        Executer {
            configuration : c.clone(),
            script : s,
            pc : 0,
            connection : StringConnection::new(c.host()),
        }
    }

    pub fn run_script(&mut self) {
        for line in &self.script {
            if line.executable() == true {
                self.connection.send(line.data());
            }
        }
    }
}
