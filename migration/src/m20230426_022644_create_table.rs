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
        if let Err(x) = try_join!(
            create_user(manager),
            create_role(manager),
            create_resource(manager),
            create_rel_role_resource(manager),
            create_rel_user_role(manager)
        ) {
            return Err(x);
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        if let Err(x) = try_join!(
            manager.drop_table(Table::drop().table(SysUser::Table).to_owned()),
            manager.drop_table(Table::drop().table(SysRole::Table).to_owned()),
            manager.drop_table(Table::drop().table(SysResource::Table).to_owned()),
            manager.drop_table(Table::drop().table(SysRelRoleResource::Table).to_owned()),
            manager.drop_table(Table::drop().table(SysRelUserRole::Table).to_owned())
        ) {
            return Err(x);
        }

        Ok(())
    }
}

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
// 创建 资源 表
async fn create_resource(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let mut tcs: TableCreateStatement = Table::create()
        .if_not_exists()
        .table(SysResource::Table)
        .col(
            ColumnDef::new(SysResource::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(SysResource::Name).string_len(20).not_null())
        .col(ColumnDef::new(SysResource::Type).char_len(1).not_null())
        .col(ColumnDef::new(SysResource::Url).string_len(256))
        .col(ColumnDef::new(SysResource::Desc).string_len(20))
        .col(
            ColumnDef::new(SysResource::OrderId)
                .tiny_integer()
                .default(0),
        )
        .to_owned();

    // 添加时间字段
    add_time_col(&mut tcs, TimeOpt::Default);

    manager.create_table(tcs.to_owned()).await
}

#[derive(Iden)]
enum SysRole {
    Table,
    Id,
    Name,
    Desc,
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


#[derive(Iden)]
enum SysRelRoleResource {
    Table,
    Id,
    RoleId,
    ResourceId,
}
// 创建 角色资源关联 表
async fn create_rel_role_resource(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let mut tcs: TableCreateStatement = Table::create()
        .if_not_exists()
        .table(SysRelRoleResource::Table)
        .col(
            ColumnDef::new(SysRelRoleResource::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(SysRelRoleResource::RoleId).integer().not_null())
        .col(ColumnDef::new(SysRelRoleResource::ResourceId).integer().not_null())
        .to_owned();

    manager.create_table(tcs.to_owned()).await
}


#[derive(Iden)]
enum SysRelUserRole {
    Table,
    Id,
    UserId,
    RoleId,
}
// 创建 用户角色关联 表
async fn create_rel_user_role(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let mut tcs: TableCreateStatement = Table::create()
        .if_not_exists()
        .table(SysRelUserRole::Table)
        .col(
            ColumnDef::new(SysRelUserRole::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(SysRelUserRole::UserId).integer().not_null())
        .col(ColumnDef::new(SysRelUserRole::RoleId).integer().not_null())
        .to_owned();

    manager.create_table(tcs.to_owned()).await
}