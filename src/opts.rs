use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

// 子命令，实现Subcommand的只能是enum枚举
#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    #[command(
        name = "jwt",
        about = "jwt token 工具，jwt由3部分组成：header、payload和signature"
    )]
    Jwt(JwtCommand),
}

// 子命令
#[derive(Debug, Subcommand)]
pub enum JwtCommand {
    #[command(name = "sign", about = "")]
    Sign(SignOpts), // Signature = HMACSHA256(base64UrlEncode(header)+"."+base64UrlEncode(payload),secret)
    #[command(name = "verify", about = "验证")]
    Verify(VerifyOpts),
}

// 命令行参数，实现Args的只能是struct结构体
#[derive(Debug, Args)]
pub struct SignOpts {
    // Payload
    #[arg(long,default_value_t=String::from("vshader"))]
    pub iss: String, // 发行人
    #[arg(long, default_value_t = 10)]
    pub exp: i64, // 到期时间=生成时间+exp(分钟)
    #[arg(long,default_value_t=String::from(""))]
    pub sub: String, // 主题
    #[arg(long,default_value_t=String::from(""))]
    pub aud: String, // 用户

    #[arg(long,default_value_t=String::from("123asd"))]
    // 指定一个密钥，对经过base64url encode后的header和payload，进行签名
    pub secret: String,
    #[arg(long,default_value_t=String::from("HS256"),help="目前支持的签名算法有HS256、HS384、HS512，默认为HS256")]
    pub alg: String,
}

// 命令行参数
#[derive(Debug, Args)]
pub struct VerifyOpts {
    #[arg(short, long)]
    pub token: String,
    #[arg(long,default_value_t=String::from("123asd"))]
    // 指定一个密钥，对经过base64url encode后的header和payload，进行签名
    pub secret: String,
    #[arg(long,default_value_t=String::from("HS256"))]
    pub alg: String,
}
