//! MIT License

//! Copyright (c) [year] [fullname]

//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:

//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.

//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.
//! 
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use serde_json;
use serde::{Serialize, Deserialize};
use crate::Error;
use crate::deploy::CommandSet;
use std::process::exit;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Config {
    pub config_dir: String,
    pub command_config_file: String,
    pub plan_files:Vec<String>,
    pub taskfiles: HashMap<String, String>,
    #[serde(skip_serializing, skip_deserializing)]
    commandpool: Option<CommandSet>
}

impl Config {
    pub fn new(config_path: Option<String>) -> Result<Config, Error> {
        static CONFIG_FILE: &str = "config.json";
        let config_path = match config_path {
            Some(a) => a,
            None => CONFIG_FILE.to_owned(),
        };
        let mut f = File::open(PathBuf::from(config_path))?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        let mut config: Config = serde_json::from_str(&s)?;
        config.load_commandpool()?;
        Ok(config)
    }

    pub fn get_commandpool(&mut self) -> CommandSet {
        if self.commandpool.is_none() {
            match self.load_commandpool() {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error loading commandpool: {:?}!", e);
                    exit(1);
                }
            };
        }
        self.commandpool.clone().unwrap()
    }

    fn load_commandpool(&mut self) -> Result<(), Error> {
        if self.commandpool.is_none() {
            let path = PathBuf::from(&self.config_dir).join(&self.command_config_file);
            self.commandpool = Some(CommandSet::from_file(path.to_str().unwrap())?);
        }
        Ok(())
    }
}