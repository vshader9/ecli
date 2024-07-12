use super::{SignOpts, VerifyOpts};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::{fmt, ops::Deref};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String, // 发行人
    iat: i64,    // 发布时间
    exp: i64,    // 到期时间=生成时间+exp(分钟)
    sub: String, // 主题
    aud: String, // 用户
}

impl Claims {
    fn new(opts: &SignOpts) -> Self {
        let iat = Utc::now().timestamp();
        let exp = opts.exp * 60 + iat;

        Self {
            iss: opts.iss.to_owned(),
            iat,
            exp,
            sub: opts.sub.to_owned(),
            aud: opts.aud.to_owned(),
        }
    }
}

impl fmt::Display for Claims {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "payload:{{
    iss:{},
    iat:{},
    exp:{},
    sub:{},
    aud:{}
}}",
            self.iss, self.iat, self.exp, self.sub, self.aud
        )
    }
}

#[derive(Debug)]
pub struct MyHeader(Header);

impl MyHeader {
    pub fn new(header: Header) -> Self {
        Self(header)
    }
}

impl Deref for MyHeader {
    type Target = Header;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for MyHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "header:{{
    alg:{:?},
    typ:{:?}
}}",
            self.alg,
            self.typ.clone().unwrap_or(String::from("JWT"))
        )
    }
}

pub fn sign(opts: &SignOpts) {
    let alg = opts.alg.parse::<Algorithm>().unwrap_or(Algorithm::HS256);
    let header = MyHeader::new(Header::new(alg));
    let claims = Claims::new(opts);
    let encoding_key = EncodingKey::from_secret(opts.secret.as_bytes());
    match encode(&header, &claims, &encoding_key) {
        Ok(token) => println!("Encoded - {:?}\n{}\n{}\n{}", &alg, &header, &claims, &token),
        Err(e) => println!("签名失败：{:?}", e),
    }
}

pub fn verify(opts: &VerifyOpts) {
    let alg = opts.alg.parse::<Algorithm>().unwrap_or(Algorithm::HS256);
    println!("{:?}", alg);
    let token = opts.token.as_str();
    let decoding_key = DecodingKey::from_secret(opts.secret.as_bytes());
    let mut validation = Validation::new(alg);
    validation.validate_aud = false;
    validation.validate_exp = false;
    match decode::<Claims>(token, &decoding_key, &validation) {
        Ok(token_data) => println!(
            "Decoded\n----------\n{}\n{}",
            MyHeader::new(token_data.header),
            &token_data.claims
        ),
        Err(e) => println!("验证失败：{:?}", e),
    }
}
