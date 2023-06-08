use crate::entity;
use crate::enums;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

// DTO ↓
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    pub id: i32,
    pub uname: String,
    pub nickname: Option<String>,
    pub sex: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub enabled: bool,
    pub created_at: Option<DateTimeUtc>,
    pub created_by: Option<i32>,
    pub updated_at: Option<DateTimeUtc>,
    pub updated_by: Option<i32>,
}

impl From<entity::sys_user::Model> for UserDto {
    fn from(value: entity::sys_user::Model) -> Self {
        UserDto {
            id: value.id,
            uname: value.uname,
            nickname: value.nickname,
            sex: value.sex,
            mobile: value.mobile,
            email: value.email,
            enabled: enums::common::EnumEnabled::from(value.enabled).into(),
            created_at: value.created_at,
            created_by: value.created_by,
            updated_at: value.updated_at,
            updated_by: value.updated_by,
        }
    }
}
