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
use crate::config::Config;
use crate::deploy::DeployPlan;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Executor {
    plans: Option<HashMap<String, DeployPlan>>,
    config: Option<Config>,
}

impl Executor {
    pub fn new() -> Self {
        return Executor {
            plans: None,
            config: None,
        }
    }

    pub fn load_context(&mut self, config_path: Option<String>) {
        let config: Config = Config::new(config_path).expect("Error reading config file!");
        self.config = Some(config);
        self.prepair_deploy_plan();
    }
    
    fn prepair_deploy_plan(&mut self) {
        let mut m: HashMap<String, DeployPlan> = HashMap::new();
        let config = self.config.clone().unwrap();
        for n in &config.plan_files {
            let path = PathBuf::from(&config.config_dir).join(n);
            let dp = DeployPlan::from_file(path.to_str().unwrap(), Some(config.clone()));
            m.insert(dp.name.clone(), dp);
        }
        self.plans = Some(m);
    }

    pub fn run_deploy_plan(&mut self, s: &str) {
        if self.plans.is_none() {
            eprintln!("Context not loaded!");
        }
        let mut m = self.plans.take().unwrap();
        match m.get_mut(s) {
            Some(p) => {
                match p.run() {
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("Error deplying plan `{}`: {:?}", &p.name, e);
                    }
                }
            }
            None => {
                eprintln!("No plan named `{}`!", s);
            },
        }
        self.plans = Some(m);
    }
}

