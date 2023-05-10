// use crate::TableCreateStatement;
use sea_orm_migration::prelude::*;

pub enum TimeOpt {
    Default,    // create_at/create_by/update_at/update_by
    AllWithStr, // create_at/create_by/create_by_str/update_at/update_by/update_by_str
    OnlyCreate, // create_at/create_by
}

#[derive(Iden)]
enum TimeCol {
    CreatedAt,
    CreatedBy,
    CreatedByStr,
    UpdatedAt,
    UpdatedBy,
    UpdatedByStr,
}

pub fn add_time_col(tcs: &mut TableCreateStatement, time_opt: TimeOpt) {
    match time_opt {
        TimeOpt::Default => {
            with_create(tcs);
            with_update(tcs);
        }
        TimeOpt::AllWithStr => {
            with_create(tcs);
            with_update(tcs);
            with_str(tcs);
        }
        TimeOpt::OnlyCreate => {
            with_create(tcs);
        }
    };
}

/**
 * [创建]
 * 为表添加 创建时间 / 创建人 字段
 */
fn with_create(tcs: &mut TableCreateStatement) -> &mut TableCreateStatement {
    tcs.col(
        ColumnDef::new(TimeCol::CreatedAt)
            .timestamp()
            .extra("COMMENT '创建时间'".to_owned()),
    )
    .col(
        ColumnDef::new(TimeCol::CreatedBy)
            .integer()
            .extra("COMMENT '创建人'".to_owned()),
    )
}

/**
 * [更新]
 * 为表添加 更新时间 / 更新人 字段
 */
fn with_update(tcs: &mut TableCreateStatement) -> &mut TableCreateStatement {
    tcs.col(
        ColumnDef::new(TimeCol::UpdatedAt)
            .timestamp()
            .extra("COMMENT '更新时间'".to_owned()),
    )
    .col(
        ColumnDef::new(TimeCol::UpdatedBy)
            .integer()
            .extra("COMMENT '更新人'".to_owned()),
    )
}

/**
 * [用户名称]
 * 为 创建/更新 添加用户名称字段
 * 当需要在表格中粗略显示用户名称时,保存一个用户名称
 */
fn with_str(tcs: &mut TableCreateStatement) -> &mut TableCreateStatement {
    tcs.col(
        ColumnDef::new(TimeCol::CreatedByStr)
            .timestamp()
            .extra("COMMENT '创建人名称'".to_owned()),
    )
    .col(
        ColumnDef::new(TimeCol::UpdatedByStr)
            .integer()
            .extra("COMMENT '更新人名称'".to_owned()),
    )
}
