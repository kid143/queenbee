# queenbee 

一个简单的命令运行工具，Rust学习项目。
本项目纯粹是因为本人学Rust刷LeetCode时觉得略显枯燥，就
自己瞎逼鼓捣出来的玩意，功能极度简单，就是跑shell命令，
目前也就在Linux系统上跑一下。用来跑一些我个人的vps上的
一些维护或者部署的脚本。

# 用法
由于是学习类的项目，推送到crate.io是不合适的，所以只提供从代码库clone
自行编译的用法。

1. 克隆本代码库
2. 运行`cargo build`

在代码库下的`target`目录下能找到对应的queenbee执行文件

命令行用法：
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

更具体的用法请参考代码，不是很复杂，估计10分钟就能看完。
*注意：本项目是个玩具项目，不要用在对于安全性和健壮性要求很高的场合，由此引发的问题本人一概不负责！*
