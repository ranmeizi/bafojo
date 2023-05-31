/*
 * @Author: boboan 360969885@qq.com
 * @Date: 2023-05-31 09:35:22
 * @LastEditors: boboan 360969885@qq.com
 * @LastEditTime: 2023-05-31 22:29:42
 * @FilePath: /bafojo/common/src/utils/jwt.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use chrono::{prelude::Utc, Duration};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    uid: String,
    exp: i64,
}

/**
 * 签发 token
 */
pub fn authorize(uid: &str) {
    let header = Header::default();

    // 计算过期时间
    let exp =get_exp();

    let claims = Claims {
        uid: uid.into(),
        exp: exp,
    };
}

/**
 * 校验 token
 */
pub fn check_access_token(token: &str) -> Result<(), String> {
    // 校验签名

    // 校验有效期

    Ok(())
}

fn get_exp() -> i64 {
    // 计算过期时间
    let duration = Duration::hours(2);
    let exp = Utc::now().checked_add_signed(duration).unwrap();

    exp.timestamp()
}
