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
use structopt::StructOpt;
use queenbee::executor:: Executor;

#[derive(Debug, StructOpt)]
#[structopt(name = "queenbee", about = "Usage of queenbee")]
struct Opt {
    #[structopt(short = "c", long = "config", help = "Config file path")]
    config: Option<String>,
    #[structopt(help = "Plan to execute")]
    plan: String
}

fn main() {
    let opt = Opt::from_args();
    let mut executor = Executor::new();
    executor.load_context(opt.config);
    executor.run_deploy_plan(&opt.plan);
}
