/*
 * @Author: boboan 360969885@qq.com
 * @Date: 2023-05-31 09:35:22
 * @LastEditors: boboan 360969885@qq.com
 * @LastEditTime: 2023-05-31 22:29:42
 * @FilePath: /bafojo/common/src/utils/jwt.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use crate::{config::CFG, error::AuthErr};
use anyhow::Result;
use chrono::{prelude::Utc, Duration};
use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub uname: String,
    pub uid:i32,
    pub exp: i64,
}

/**
 * 签发 token
 */
pub fn authorize(uname: &str,uid:i32) -> Result<(String, i64)> {
    let header = Header::new(Algorithm::RS256);

    // 计算过期时间
    let exp = get_exp();

    let claims = Claims {
        uname: uname.into(),
        uid,
        exp: exp,
    };

    let sign = get_sign(&header, &claims)?;

    Ok((sign, exp))
}

fn get_sign(header: &Header, claims: &Claims) -> Result<String> {
    let key = EncodingKey::from_rsa_pem(include_bytes!("cert/private.pem"))?;

    let token = encode(&header, &claims, &key)?;

    Ok(token)
}

/**
 * 校验 token
 */
pub fn check_access_token(token: &str) -> Result<Claims> {
    // 获取当前时间
    let now = Utc::now().timestamp();

    // 校验签名
    let key = DecodingKey::from_rsa_pem(include_bytes!("cert/public.pem"))?;

    println!("res ok");

    let res = decode::<Claims>(&token, &key, &Validation::new(Algorithm::RS256));

    if res.is_err() {
        // 无效token
        return Err(AuthErr::InvalidToken.into());
    }

   let claims = res.unwrap().claims;


    // 校验有效期
    let exp = claims.exp;

    if now > exp {
        // 超过有效期
        return Err(AuthErr::ExpiredToken.into());
    }

    Ok(claims)
}

fn get_exp() -> i64 {
    let exp_min = CFG.jwt.exp_min.clone();
    // 计算过期时间
    let duration = Duration::minutes(exp_min);
    let exp = Utc::now().checked_add_signed(duration).unwrap();

    exp.timestamp()
}
