# queenbee 

A simple command executing tool from scratch for self-learning Rust。
I build this project simply because get bored of hours coding on LeetCode
without any project that can help me familiarize the standard lib in Rust.
This project is not designed to run on multiple platform, I just developed
and tested on Linux system (specifically, Ubuntu). It is used on my own vps
for running some scripts.

# Usage

It is just a learning project so I won't upload it to crate.io. You have to
clone this repository to and build it with `cargo` yourself.

1. Clone this repo.
2. Run `cargo build` inside the repo folder.

find the executable file in the `target` folder.

Command line usage：
```
queenbee 0.1.0
Usage of queenbee

USAGE:
    queenbee [OPTIONS] <plan>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config>    Config file path

ARGS:
    <plan>    Plan to execute
```

More detail please refer to the code since it is not a very big project and
you should be able to finish reading very soon.

*Caution: This is just a toy project, please DO NOT use in critical scene, I
am NOT responsible for any problem caused by using this project!*