use clap::Parser;

//定义自动生成密码子命令的参数结构体
#[derive(Debug, Parser)]
pub struct GenPassOpts {
    //密码长度参数，默认值16
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,
    //是否包含特殊字符参数，默认值true
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,
    //是否包含小写字母参数，默认值true
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
    //是否包含数字参数，默认值true
    #[arg(long, default_value_t = true)]
    pub numbers: bool,
    //是否包含符号参数，默认值true
    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}
