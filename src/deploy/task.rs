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
use std::path::PathBuf;
use serde_json;
use super::{Serialize, Deserialize, File, Read};
use super::command::{ExecutableCommand, CommandSet};
use crate::Error;
use crate::config::Config;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct DeployTask {
    pub name: String,
    pub commandseq: Vec<usize>,
    #[serde(skip_serializing, skip_deserializing)]
    commands: Option<CommandSet>,
    #[serde(skip_serializing, skip_deserializing)]
    config: Option<Config>,
}

impl DeployTask {

    pub fn from_file(s: &str, config: Option<Config>) -> Result<Self, Error> {
        let mut dt = DeployTask::load_from_file(s.to_owned())?;
        dt.config = config;
        Ok(dt)
    }

    fn load_from_file(s: String) -> Result<DeployTask, Error> {
        let mut f = File::open(PathBuf::from(&s))?;
        let mut b: String = String::new();
        f.read_to_string(&mut b)?;
        let task: DeployTask = serde_json::from_str(&b)?;
        return Ok(task);
    }

    pub fn load_commands(&mut self, cs: &CommandSet) {
        self.commands = cs.select_command_by_index(&self.commandseq);
    }

    pub fn as_commands(&mut self) -> Result<Vec<ExecutableCommand>, Error> {
        if self.commands.is_none() {
            return Err(Error::from("No command in task, forget to load commands? Or there is someting wrong with the config!"));
        }
        Ok(self.commands.take().unwrap().as_commands())
    }

    pub fn run(&mut self) -> Result<(), Error> {
        if self.commands.is_none() {
            let config = self.config.as_mut().unwrap();
            let cs = config.get_commandpool();        
            self.load_commands(&cs);
        }
        println!("Running task: {}", &self.name);
        let commands = self.as_commands()?;
        for c in commands {
            println!("Executing: `{} {:?}`", c.command, c.args);
            let output = c.as_command().output().expect("Error executing commands!");
            println!("{}", String::from_utf8(output.stdout).unwrap());
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::DeployTask;
    use crate::deploy::command::CommandSet;
    use crate::config::Config;
    #[test]
    fn from_file() {
        let config = Some(Config::new(Some("./testresource/config.json".to_owned())).unwrap());
        let dt = DeployTask::from_file("./testresource/testtasks.json", config).unwrap();
        // let cs = CommandSet::from_file("./testresource/multiplecommands.json");
        let sq: Vec<usize> = vec![1, 2, 3, 4, 5];
        assert_eq!(dt.commandseq, sq);
        assert_eq!(dt.name, "Task1".to_owned());
    }

    #[test]
    fn test_load_commands() {
        let config = Some(Config::new(Some("./testresource/config.json".to_owned())).unwrap());
        let mut dt = DeployTask::from_file("./testresource/testtasks.json", config).unwrap();
        let cs = CommandSet::from_file("./testresource/multiplecommands.json").unwrap();
        dt.load_commands(&cs);
        for (i, c) in dt.commands.take().unwrap().as_commands().iter().enumerate() {
            assert_eq!(c.idx, i + 1);
        }
    }

    #[test]
    fn test_load_from_mal_file() {
        let dt = DeployTask::load_from_file("./testresource/faultjson.json".to_owned());
        assert!(dt.is_err());
    }
}
