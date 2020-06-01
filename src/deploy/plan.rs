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
    File,
    Read,
    Serialize,
    Deserialize,
    task::DeployTask
};
use crate::Error;
use crate::config::Config;
use serde_json;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct DeployPlan {
    pub name: String,
    pub tasknames: Vec<String>,
    #[serde(skip_serializing, skip_deserializing)]
    tasks: Option<Vec<DeployTask>>,
    #[serde(skip_serializing, skip_deserializing)]
    config: Option<Config>,
}

impl DeployPlan {

    pub fn from_file(s: &str, config: Option<Config>) -> DeployPlan {
        let mut dp = DeployPlan::load_from_file(s.to_owned())
                            .expect(&format!("Error loading DeployPlan `{}`!", s));
        dp.config = config;
        dp
    }

    pub fn run(&mut self) -> Result<(), Error> {
        if self.tasks.is_none() {
            self.load_task()?
        }
        for t in self.tasks.take().as_mut().unwrap() {
            t.run()?
        }
        Ok(())
    }

    fn load_task(&mut self) -> Result<(), Error> {
        let config = self.config.clone().unwrap();
        let task_files = &config.taskfiles;
        let config_dir = &config.config_dir;
        let mut v: Vec<DeployTask> = Vec::with_capacity(self.tasknames.len());
        for t in &self.tasknames {
            match task_files.get(t) {
                Some(f) => {
                    let path = PathBuf::from(config_dir).join(f);
                    let task = DeployTask::from_file(path.to_str().unwrap(), Some(config.clone()))?;
                    v.push(task);
                },
                None => {},
            }
        }
        self.tasks = Some(v);
        Ok(())
    }

    fn load_from_file(s: String) -> Result<DeployPlan, Error> {
        let mut f = File::open(PathBuf::from(&s))?;
        let mut b: String = String::new();
        f.read_to_string(&mut b)?;
        let task: DeployPlan = serde_json::from_str(&b)?;
        return Ok(task);
    }
}

#[cfg(test)]
mod test {
    use super::DeployPlan;
    use crate::config::Config;
    #[test]
    fn test_from_file() {
        let config = Some(Config::new(Some("./testresource/config.json".to_owned())).unwrap());
        let dp = DeployPlan::from_file("./testresource/testplan.json", config);
        assert_eq!(dp.name, "Plan1".to_owned());
        assert_eq!(dp.tasknames.len(), 1);
        assert_eq!(dp.tasknames[0], "Task1".to_owned());
    }

    #[test]
    fn test_load_from_mal_file() {
        let dp = DeployPlan::load_from_file("./testresource/faultjson.json".to_owned());
        assert!(dp.is_err());
    }
}