use bfj_common::dto::system::UserDto;
use sea_orm::prelude::DateTimeUtc;
use chrono::prelude::Utc;

pub struct CreateInfo {
    pub created_at:Option<DateTimeUtc>,
    pub created_by:Option<i32>,
    pub created_str:Option<String>
}

pub struct UpdateInfo {
    pub updated_at:Option<DateTimeUtc>,
    pub updated_by:Option<i32>,
    pub updated_str:Option<String>
}

fn get_uid_and_name(userinfo: &Option<UserDto>)->(Option<i32>,Option<String>){
    match userinfo {
        Some(userinfo) => {
            (Some(userinfo.id),userinfo.nickname.clone())
        },
        None=>  (None,None)
    }
}

pub fn get_create_info(userinfo: &Option<UserDto>)->CreateInfo{
    let (id,name) = get_uid_and_name(userinfo);
    CreateInfo {
        created_at:Some(Utc::now()),
        created_by:id,
        created_str:name
    }
}

pub fn get_update_info(userinfo: &Option<UserDto>)->UpdateInfo{
    let (id,name) = get_uid_and_name(userinfo);
    UpdateInfo {
        updated_at:Some(Utc::now()),
        updated_by:id,
        updated_str:name
    }
}
