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
pub mod command;
pub mod task;
pub mod plan;

extern crate serde;
extern crate serde_json;

use std::process::Command;
use std::fs::File;
use std::io::{Read};
use std::cmp::{PartialOrd, Ord};
use serde::{Serialize, Deserialize};

pub use command::{ExecutableCommand, CommandSet};
pub use task::DeployTask;
pub use plan::DeployPlan;