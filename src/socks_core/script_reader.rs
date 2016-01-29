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
use std::io::{self, Error};
use std::fs::File;
use regex::Regex;

use socks_core::configuration::Configuration;
use socks_core::script_line::ScriptLine;


pub struct ScriptReader {
    configuration : Configuration,
}


impl ScriptReader {
    pub fn new(config : &Configuration) -> ScriptReader {
        ScriptReader {
            configuration : config.clone()
        }
    }

    pub fn prepare_script(&self, path : &String) -> Vec<ScriptLine> {
        let mut script = Vec::<ScriptLine>::new();

        let content = match self.read_content(path) {
            Ok(c) => c,
            Err(_) => panic!("Can't read file"),
        };

        let mut count : u64 = 0;

        for c in content {
            let mut line = ScriptLine::new_from_line(&c);

            line.set_number(&count);

            if self.match_criterias(&c, self.configuration.non_executable_criterias()) {
                line.set_executable(&true);
            }

            if self.match_criterias(&c, self.configuration.query_criterias()) {
                line.set_query(&true);
            }

            script.push(line);
            count += 1;
        }

        script
    }

    fn read_content(&self, path : &String) -> Result<Vec<String>, io::Error> {
        let mut f = try!(File::open(path));
        let mut s = String::new();
        let mut result = Vec::<String>::new();

        try!(f.read_to_string(&mut s));

        let p = s.split('\n');

        for part in p {
            result.push(part.to_string());
        }

        Ok(result)
    }

    fn match_criterias(&self, line : &String, criterias : &Vec<String>) -> bool {
        if line == "" {
            return false
        }

        for c in criterias {
            let re = Regex::new(c).unwrap();

            if re.find(line) != None {
                return false
            }
        }

        true
    }
}
