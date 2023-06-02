use crate::{PageData, PageParams};
use anyhow::{anyhow, Result};
use bfj_common::{dto::system::UserDto, entity::sys_user, error::AuthErr};
use chrono::prelude::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use serde::Deserialize;

pub struct Query {}
pub struct Mutation {}

impl Query {
    /**
     * 使用用户名 获取 psw 和 salt
     * @returns {(psw:String,salt:String)}
     */
    pub async fn get_psw_by_uname(
        db: &DatabaseConnection,
        uname: &str,
    ) -> Result<(String, String)> {
        let user = sys_user::Entity::find()
            .filter(sys_user::Column::Uname.eq(uname))
            .one(db)
            .await?;

        match user {
            Some(m) => Ok((m.psw, m.salt)),
            _ =>Err(anyhow!("")),
        }
    }
}

impl Mutation {}

/**
 * 登陆参数
 */
#[derive(Debug, Deserialize)]
pub struct LoginParams {
    pub uname: String,
    pub psw: String,
}
