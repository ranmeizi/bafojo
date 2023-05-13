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
    Enabled,
    Col1,
    Col2,
    Col3,
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
                .primary_key()
                .auto_increment(),
        )
        .col(
            ColumnDef::new(SysUser::Uname)
                .string_len(20)
                .not_null()
                .extra("COMMENT '用户名'".to_owned()),
        )
        .col(
            ColumnDef::new(SysUser::Psw)
                .char_len(20)
                .not_null()
                .extra("COMMENT '加密后的密码'".to_owned()),
        )
        .col(
            ColumnDef::new(SysUser::Nickname)
                .string_len(20)
                .extra("COMMENT '昵称'".to_owned()),
        )
        .col(
            ColumnDef::new(SysUser::Sex)
                .char_len(1)
                .extra("COMMENT '性别 0-未知,1-男,2-女'".to_owned()),
        )
        .col(
            ColumnDef::new(SysUser::Mobile)
                .string_len(20)
                .extra("COMMENT '手机号'".to_owned()),
        )
        .col(
            ColumnDef::new(SysUser::Email)
                .string_len(100)
                .extra("COMMENT '邮箱'".to_owned()),
        )
        .col(
            ColumnDef::new(SysUser::Enabled)
                .boolean()
                .default(true)
                .extra("COMMENT '启用状态'".to_owned()),
        )
        .col(
            ColumnDef::new(SysUser::Col1)
                .string_len(200)
                .extra("COMMENT '备用字段1'".to_owned()),
        )
        .col(
            ColumnDef::new(SysUser::Col2)
                .string_len(200)
                .extra("COMMENT '备用字段2'".to_owned()),
        )
        .col(
            ColumnDef::new(SysUser::Col3)
                .string_len(200)
                .extra("COMMENT '备用字段3'".to_owned()),
        )
        .to_owned();

    // 添加时间字段
    add_time_col(&mut tcs, TimeOpt::Default);

    manager.create_table(tcs.to_owned()).await
}

#[derive(Iden)]
enum SysResource {
    Table,
    Code,
    Parent,
    Name,
    Type,
    Url,
    Title,
    Desc,
    Order,
    Enabled,
}
// 创建 资源 表
async fn create_resource(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let mut tcs: TableCreateStatement = Table::create()
        .if_not_exists()
        .table(SysResource::Table)
        .col(
            ColumnDef::new(SysResource::Code)
                .string_len(30)
                .not_null()
                .primary_key()
                .extra("COMMENT '权限码'".to_owned()),
        )
        .col(
            ColumnDef::new(SysResource::Parent)
                .string_len(30)
                .not_null()
                .extra("COMMENT '父级权限码'".to_owned()),
        )
        .col(
            ColumnDef::new(SysResource::Name)
                .string_len(20)
                .not_null()
                .extra("COMMENT '资源名称'".to_owned()),
        )
        .col(
            ColumnDef::new(SysResource::Type)
                .char_len(1)
                .not_null()
                .extra("COMMENT '资源类型 1-普通,2-菜单'".to_owned()),
        )
        .col(
            ColumnDef::new(SysResource::Title)
                .string_len(20)
                .extra("COMMENT '菜单/页面名称 当 type = 菜单 时才有效'".to_owned()),
        )
        .col(
            ColumnDef::new(SysResource::Url)
                .string_len(256)
                .extra("COMMENT '菜单地址 当 type = 菜单 时才有效'".to_owned()),
        )
        .col(
            ColumnDef::new(SysResource::Desc)
                .string_len(20)
                .extra("COMMENT '资源描述'".to_owned()),
        )
        .col(
            ColumnDef::new(SysResource::Order)
                .small_integer()
                .default(0)
                .extra("COMMENT '排序字段 默认为0'".to_owned()),
        )
        .col(
            ColumnDef::new(SysResource::Enabled)
                .boolean()
                .default(true)
                .extra("COMMENT '启用状态'".to_owned()),
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
        .col(
            ColumnDef::new(SysRole::Name)
                .string_len(20)
                .not_null()
                .extra("COMMENT '角色名称'".to_owned()),
        )
        .col(
            ColumnDef::new(SysRole::Desc)
                .string_len(100)
                .not_null()
                .extra("COMMENT '角色描述'".to_owned()),
        )
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
    let tcs: TableCreateStatement = Table::create()
        .if_not_exists()
        .table(SysRelRoleResource::Table)
        .col(
            ColumnDef::new(SysRelRoleResource::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(SysRelRoleResource::RoleId)
                .integer()
                .not_null()
                .extra("COMMENT '角色id'".to_owned()),
        )
        .col(
            ColumnDef::new(SysRelRoleResource::ResourceId)
                .integer()
                .not_null()
                .extra("COMMENT '资源id'".to_owned()),
        )
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
    let tcs: TableCreateStatement = Table::create()
        .if_not_exists()
        .table(SysRelUserRole::Table)
        .col(
            ColumnDef::new(SysRelUserRole::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(SysRelUserRole::UserId)
                .integer()
                .not_null()
                .extra("COMMENT '用户id'".to_owned()),
        )
        .col(
            ColumnDef::new(SysRelUserRole::RoleId)
                .integer()
                .not_null()
                .extra("COMMENT '角色id'".to_owned()),
        )
        .to_owned();

    manager.create_table(tcs.to_owned()).await
}
