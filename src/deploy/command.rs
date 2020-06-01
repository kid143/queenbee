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
use super::{
    Serialize,
    Deserialize,
    PartialOrd,
    Ord,
    Command,
    File,
    Read,
    PathBuf
};
use crate::Error;
use serde_json;
use std::cmp::Ordering;


#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct ExecutableCommand {
    pub idx: usize,
    pub command: String,
    pub args: Vec<String>,
}

impl ExecutableCommand {
    pub fn as_command(&self) -> Command {
        let mut c = Command::new(&self.command);
        c.args(&self.args);
        return c;
    }
}

impl PartialOrd for ExecutableCommand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ExecutableCommand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.idx.cmp(&other.idx)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CommandSet {
    commands: Vec<ExecutableCommand>,
}

impl CommandSet {
    pub fn new(v: Vec<ExecutableCommand>) -> Self {
        CommandSet {
            commands: v,
        }
    }

    pub fn from_file(s: &str) -> Result<Self, Error> {
        let commands = Self::load_from_file(s.to_owned())?;
        Ok(CommandSet { 
            commands: commands
        })
    }

    pub fn select_command_by_index(&self, index: &[usize]) -> Option<Self> {
        let mut v: Vec<ExecutableCommand> = Vec::with_capacity(index.len());
        for i in index {
            if i - 1 >= self.commands.len() {
                panic!("Index out of bounds, no such command with index `{}` in current commandset!");
            } 
            v.push(self.commands[i - 1].clone());
        }
        if v.len() == 0 {
            None
        } else {
            Some(Self::new(v))
        }
    }

    fn load_from_file(s: String) -> Result<Vec<ExecutableCommand>, Error> {
        let mut f = File::open(PathBuf::from(&s))?;
        let mut b: String = String::new();
        f.read_to_string(&mut b)?;
        let mut ecs: Vec<ExecutableCommand> = serde_json::from_str(&b)?;
        ecs.sort_by(|a, b| { a.cmp(b) });
        return Ok(ecs);
    }

    pub fn as_commands(&self) -> Vec<ExecutableCommand> {
        let mut v: Vec<ExecutableCommand> = Vec::with_capacity(self.commands.len());
        for i in self.commands.iter() {
            v.push(i.clone());
        }
        v
    }
}

#[cfg(test)]
mod test {
    use super::CommandSet;
    #[test]
    fn test_from_file() {
        let cs = CommandSet::from_file("./testresource/testcommands.json").unwrap();
        assert_eq!(cs.commands.len(), 1);
        let ec = &cs.commands[0];
        assert_eq!(ec.idx, 1);
    }

    #[test]
    fn test_load_from_file() {
        let cs = CommandSet::load_from_file("./testresource/testcommands.json".to_owned());
        assert!(cs.is_ok());
    }

    #[test]
    fn test_load_from_mal_file() {
        let cs = CommandSet::load_from_file("./testresource/faultjson.json".to_owned());
        assert!(cs.is_err());
    }

    #[test]
    fn test_select_command_by_index() {
        let cs = CommandSet::from_file("./testresource/multiplecommands.json").unwrap();
        let cs2 = cs.select_command_by_index(&[3,5,6]).unwrap();
        let cm = cs2.commands;
        assert_eq!(cm[0].idx, 3);
        assert_eq!(cm[1].idx, 5);
        assert_eq!(cm[2].idx, 6);
    }

    #[test]
    #[should_panic]
    fn test_select_command_by_index_panic() {
        let cs = CommandSet::from_file("./testresource/multiplecommands.json").unwrap();
        // No command with idx over 6, so 7 triggered panic.
        let cs2 = cs.select_command_by_index(&[3,5,7]).unwrap();
        let cm = cs2.commands;
        assert_eq!(cm[0].idx, 3);
        assert_eq!(cm[1].idx, 5);
        assert_eq!(cm[2].idx, 7);
    }
}
