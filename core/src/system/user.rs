use crate::crypto::{gen_salt, into_md5_psw};
use crate::entity::sys_user;
use crate::{PageData, PageParams};
use anyhow::{anyhow, Result};
use bfj_common::{dto::system::UserDto, error::CustErr};
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
     * 获取列表
     */
    pub async fn get_user_list(
        db: &DatabaseConnection,
        page_params: PageParams,
        params: QueryUserListParams,
    ) -> Result<PageData<UserDto>> {
        let page_num = page_params.page_num.unwrap_or(1);
        let page_size = page_params.page_size.unwrap_or(20);

        // 组装查询条件
        let mut s = sys_user::Entity::find();

        if let Some(uname) = params.uname {
            s = s.filter(sys_user::Column::Uname.like(&format!("%{uname}%")));
        }

        if let Some(nickname) = params.nickname {
            s = s.filter(sys_user::Column::Nickname.like(&format!("%{nickname}%")));
        }

        if let Some(email) = params.email {
            s = s.filter(sys_user::Column::Email.like(&format!("%{email}%")));
        }

        if let Some(mobile) = params.mobile {
            s = s.filter(sys_user::Column::Mobile.like(&format!("%{mobile}%")));
        }

        // 分页
        let total = s.clone().count(db).await?;
        let paginator = s
            .order_by_asc(sys_user::Column::CreatedAt)
            .paginate(db, page_size);
        let total_pages = paginator.num_pages().await?;
        let list: Vec<UserDto> = paginator
            .fetch_page(page_num - 1)
            .await?
            .iter()
            .map(|m| m.clone().into())
            .collect();

        Ok(PageData {
            record: list,
            total: total,
            current: page_num,
            page_size: page_size,
            total_pages: total_pages,
        })
    }

    /**
     * 按id查询
     */
    pub async fn find_user_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<sys_user::Model>> {
        let user = sys_user::Entity::find_by_id(id).one(db).await?;

        Ok(user)
    }

    pub async fn check_unique_uname(db: &DatabaseConnection, uname: &str) -> Result<bool> {
        let count = sys_user::Entity::find()
            .filter(sys_user::Column::Uname.eq(uname))
            .count(db)
            .await?;
        Ok(count > 0)
    }
}

impl Mutation {
    /**
     * 管理员创建user 默认密码 111111
     */
    pub async fn create_user(
        db: &DatabaseConnection,
        params: AddUserParams,
    ) -> Result<sys_user::Model> {
        // 判断 uname 是否重复
        if Query::check_unique_uname(db, &params.uname).await? {
            // 响应错误
            return Err(CustErr::AppRuleError("用户名不可以重复".to_owned()).into());
        }

        let salt = gen_salt();
        let psw = into_md5_psw("111111", &salt);

        let user = sys_user::ActiveModel {
            uname: Set(params.uname.to_owned()),
            salt: Set(salt),
            psw: Set(psw),
            sex: Set(params.sex),
            nickname: Set(params.nickname),
            email: Set(params.email),
            mobile: Set(params.mobile),
            created_at: Set(Some(Utc::now())),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(user)
    }

    /**
     * 更新
     */
    pub async fn update_user(
        db: &DatabaseConnection,
        params: UpdateResourceParams,
    ) -> Result<sys_user::Model> {
        let user: Option<sys_user::Model> = sys_user::Entity::find_by_id(params.id).one(db).await?;

        // Into ActiveModel
        let mut user: sys_user::ActiveModel = user.unwrap().into();

        user.nickname = Set(params.nickname);
        user.sex = Set(params.sex);
        user.mobile = Set(params.mobile);
        user.email = Set(params.email);

        // 更新修改时间
        user.updated_at = Set(Some(Utc::now()));

        let user: sys_user::Model = user.update(db).await?;

        Ok(user)
    }

    /**
     * 删除 尽可能不用删除，而是禁用
     */
    pub async fn delete_user_by_id(db: &DatabaseConnection, id: i32) -> Result<()> {
        sys_user::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }

    /**
     * 禁用
     */
    pub async fn user_enabled(
        db: &DatabaseConnection,
        params: EnabledParams,
    ) -> Result<sys_user::Model> {
        let user: Option<sys_user::Model> = sys_user::Entity::find_by_id(params.id).one(db).await?;

        // Into ActiveModel
        let mut user: sys_user::ActiveModel = user.unwrap().into();

        user.enabled = Set(params.enabled);
        // 更新修改时间
        user.updated_at = Set(Some(Utc::now()));

        let user: sys_user::Model = user.update(db).await?;

        Ok(user)
    }
}

/**
 * 创建用户参数
 */
#[derive(Debug, Deserialize)]
pub struct AddUserParams {
    uname: String,
    nickname: Option<String>,
    sex: Option<String>,
    mobile: Option<String>,
    email: Option<String>,
}

/**
 * 更新用户参数
 */
#[derive(Debug, Deserialize)]
pub struct UpdateResourceParams {
    id: i32,
    nickname: Option<String>,
    sex: Option<String>,
    mobile: Option<String>,
    email: Option<String>,
}

/**
 * 启用参数
 */
#[derive(Debug, Deserialize)]
pub struct EnabledParams {
    id: i32,
    enabled: i8,
}

/**
 * 资源筛选参数
 */
#[derive(Debug, Deserialize)]
pub struct QueryUserListParams {
    uname: Option<String>,
    nickname: Option<String>,
    mobile: Option<String>,
    email: Option<String>,
}
