use crate::utils::{add_time_col, TimeOpt};
use futures::try_join;
use sea_orm_migration::prelude::*;
use sea_query::backend::MysqlQueryBuilder;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        if let Err(x) = try_join!(create_user(manager), create_role(manager)) {
            return Err(x);
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        if let Err(x) = try_join!(
            manager.drop_table(Table::drop().table(SysUser::Table).to_owned()),
            manager.drop_table(Table::drop().table(SysRole::Table).to_owned())
        ) {
            return Err(x);
        }

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum SysUser {
    Table,
    Id,
    Uname,
    Psw,
    Nickname,
    Sex,
    Mobile,
    Email,
}

#[derive(Iden)]
enum SysRelUserRole {
    Table,
    Id,
    Name,
    Desc,
}

#[derive(Iden)]
enum SysRole {
    Table,
    Id,
    Name,
    Desc,
}

#[derive(Iden)]
enum SysRelRoleResource {
    Table,
    Id,
    Name,
    Desc,
}

#[derive(Iden)]
enum SysResource {
    Table,
    Id,
    Name,
    Type,
    Url,
    Desc,
    OrderId,
}

// 创建 用户 表
async fn create_user(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let mut tcs: TableCreateStatement = Table::create()
        .if_not_exists()
        .table(SysUser::Table)
        .col(
            ColumnDef::new(SysUser::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(SysUser::Uname).string_len(20).not_null())
        .col(ColumnDef::new(SysUser::Psw).char_len(20).not_null())
        .col(ColumnDef::new(SysUser::Nickname).string_len(20))
        .col(ColumnDef::new(SysUser::Sex).char_len(1))
        .col(ColumnDef::new(SysUser::Mobile).string_len(20))
        .col(ColumnDef::new(SysUser::Email).string_len(100))
        .to_owned();

    // 添加时间字段
    add_time_col(&mut tcs, TimeOpt::Default);

    manager.create_table(tcs.to_owned()).await
}

// 创建 角色 表
async fn create_role(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let mut tcs: TableCreateStatement = Table::create()
        .if_not_exists()
        .table(SysRole::Table)
        .col(
            ColumnDef::new(SysRole::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(SysRole::Name).string_len(20).not_null())
        .col(ColumnDef::new(SysRole::Desc).string_len(100).not_null())
        .to_owned();

    // 添加时间字段
    add_time_col(&mut tcs, TimeOpt::Default);

    manager.create_table(tcs.to_owned()).await
}
