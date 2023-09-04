use crate::constants::DATETIME_FORMAT_STRING;
use crate::entity;
use crate::enums;
use chrono::{DateTime, FixedOffset, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::LinkedList;
use std::cell::RefCell;


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
    pub created_at: Option<String>,
    pub created_by: Option<i32>,
    pub updated_at: Option<String>,
    pub updated_by: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ResourceDto {
    pub id: i32,
    pub code: String,
    pub parent: String,
    pub name: String,
    pub r#type: String,
    pub title: Option<String>,
    pub url: Option<String>,
    pub desc: Option<String>,
    pub created_at: Option<String>,
    pub created_by: Option<i32>,
    pub updated_at: Option<String>,
    pub updated_by: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ResourceNodeDto {
    pub id: i32,
    pub code: String,
    pub parent: String,
    pub name: String,
    pub r#type: String,
    pub title: Option<String>,
    pub url: Option<String>,
    pub desc: Option<String>,
    pub created_at: Option<String>,
    pub created_by: Option<i32>,
    pub updated_at: Option<String>,
    pub updated_by: Option<i32>,
    pub children: RefCell<Vec<ResourceNodeDto>>,
    // pub children: Vec<ResourceNodeDto>,

}

fn format_datetime(option_datetime: Option<DateTime<Utc>>) -> Option<String> {
    if option_datetime.is_none() {
        None
    } else {
        let utc_datetime: DateTime<Utc> = option_datetime.unwrap();
        let offset = FixedOffset::east(3600 * 8); // UTC+08:00
        let local_time = utc_datetime.with_timezone(&offset);
        Some(local_time.format(DATETIME_FORMAT_STRING).to_string())
    }
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
            created_at: format_datetime(value.created_at),
            created_by: value.created_by,
            updated_at: format_datetime(value.updated_at),
            updated_by: value.updated_by,
        }
    }
}

impl From<entity::sys_resource::Model> for ResourceDto {
    fn from(value: entity::sys_resource::Model) -> Self {
        ResourceDto {
            id: value.id,
            code: value.code,
            parent: value.parent,
            name: value.name,
            r#type: value.r#type,
            title: value.title,
            url: value.url,
            desc: value.desc,
            created_at: format_datetime(value.created_at),
            created_by: value.created_by,
            updated_at: format_datetime(value.updated_at),
            updated_by: value.updated_by,
        }
    }
}

impl From<ResourceDto> for ResourceNodeDto {
    fn from(value: ResourceDto) -> Self {
        let ResourceDto {
            id,
            code,
            parent,
            name,
            r#type,
            title,
            url,
            desc,
            created_at,
            created_by,
            updated_at,
            updated_by
        } = value;

        ResourceNodeDto{
            id,
            code,
            parent,
            name,
            r#type,
            title,
            url,
            desc,
            created_at,
            created_by,
            updated_at,
            updated_by,
            children:RefCell::new(vec![]),
            // children:vec![]
        }
    }
}
