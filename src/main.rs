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

extern crate regex;

mod socks_core;
use socks_core::configuration::Configuration;
use socks_core::script_reader::ScriptReader;
use socks_core::executer::Executer;

fn main() {
    println!("socks 0.0.0");

    let config = Configuration::new(
        &vec![r"^#".to_string(), r"^$".to_string()],
        &vec![r"\?".to_string()],
        &"localhost:40001".to_string(),
        &0xa
        );
    let reader = ScriptReader::new(&config);

    let script = reader.prepare_script(&"test.txt".to_string());
    let mut executer = Executer::new(&config, script);
    executer.run_script();
}
