use bfj_common::entity::sys_user;
use bfj_core::crypto::{gen_salt, into_md5_psw};
use bfj_core::util::mutation;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{
    query::*, ActiveModelTrait, DatabaseBackend, DatabaseTransaction, ExecResult, Set,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        let db = manager.get_connection();
        let transaction = db.begin().await?;

        add_default_user(&transaction).await?;

        transaction.commit().await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        let db = manager.get_connection();

        // 清空 user 表
        db.execute(Statement::from_string(
            DatabaseBackend::MySql,
            "DELETE FROM sys_user WHERE 1=1;".to_owned(),
        ))
        .await?;

        Ok(())
    }
}

/**
 * 创建默认用户
 */
async fn add_default_user(transaction: &DatabaseTransaction) -> Result<(), DbErr> {
    let create_info = mutation::get_create_info(&None);

    let salt = gen_salt();
    let psw = into_md5_psw("111111", &salt);

    let user = sys_user::ActiveModel {
        uname: Set("yuebuqun".to_owned()),
        salt: Set(salt),
        psw: Set(psw),
        sex: Set(Some("1".to_owned())),
        nickname: Set(Some("岳不群".to_owned())),
        email: Set(Some("ybq@huashan.com".to_owned())),
        mobile: Set(None),
        created_at: Set(create_info.created_at),
        created_by: Set(create_info.created_by),
        ..Default::default()
    };

    user.insert(transaction).await?;

    Ok(())
}
