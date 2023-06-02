use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginSuccDto {
    pub access_token: String,
    pub refresh_token: String,
    pub expire: i64,
    //todo userinfo
}
