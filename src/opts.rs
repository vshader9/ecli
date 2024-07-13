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
        name = "jwt", // jwt由3部分组成：header、payload和signature
        about = "根据指定的算法，生成jwt token"
    )]
    Jwt(JwtCommand),
}

// 子命令
#[derive(Debug, Subcommand)]
pub enum JwtCommand {
    #[command(
        name = "sign",
        about = "根据不同的算法生成jwt token，目前支持的有HS256、HS384、HS512"
    )]
    Sign(SignOpts), // Signature = HMACSHA256(base64UrlEncode(header)+"."+base64UrlEncode(payload),secret)
    #[command(name = "verify", about = "验证jwt token 是否有效")]
    Verify(VerifyOpts),
}

// 命令行参数，实现Args的只能是struct结构体
#[derive(Debug, Args)]
pub struct SignOpts {
    // Payload
    #[arg(long,default_value_t=String::from("vshader"),help="发行人")]
    pub iss: String, // 发行人
    #[arg(
        long,
        default_value_t = 10,
        help = "到期时间，单位为分钟，默认为10分钟"
    )]
    pub exp: i64, // 到期时间=生成时间+exp(分钟)
    #[arg(long,default_value_t=String::from(""),help="主题")]
    pub sub: String, // 主题
    #[arg(long,default_value_t=String::from(""),help="用户")]
    pub aud: String, // 用户

    #[arg(long,default_value_t=String::from("123asd"),help="密钥, 打死都不能说，但是别忘了")]
    // 指定一个密钥，对经过base64url encode后的header和payload，进行签名
    pub secret: String,
    #[arg(long,default_value_t=String::from("HS256"),help="签名算法，目前支持的有HS256、HS384、HS512，默认为HS256")]
    pub alg: String,
}

// 命令行参数
#[derive(Debug, Args)]
pub struct VerifyOpts {
    #[arg(short, long, help = "需要验证的jwt token")]
    pub token: String,
    #[arg(long,default_value_t=String::from("123asd"),help="密钥, 使用生成token时相同的密钥进行验证")]
    // 指定一个密钥，对经过base64url encode后的header和payload，进行签名
    pub secret: String,
    #[arg(long,default_value_t=String::from("HS256"),help="签名算法，使用生成token时相同的算法进行验证")]
    pub alg: String,
}
