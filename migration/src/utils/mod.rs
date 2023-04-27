// use crate::TableCreateStatement;
use sea_orm_migration::prelude::*;

pub enum TimeOpt {
    Default,    // create_at/create_by/update_at/update_by
    AllWithStr, // create_at/create_by/create_by_str/update_at/update_by/update_by_str
    OnlyCreate, // create_at/create_by
}

#[derive(Iden)]
enum TimeCol {
    CreateAt,
    CreateBy,
    CreateByStr,
    UpdateAt,
    UpdateBy,
    UpdateByStr,
}

pub fn add_time_col(
    tcs: &mut TableCreateStatement,
    time_opt: TimeOpt,
) {
    match time_opt {
        TimeOpt::Default => {
            with_create(tcs);
            with_update(tcs);
        },
        TimeOpt::AllWithStr => {
            with_create(tcs);
            with_update(tcs);
            with_str(tcs);
        },
        TimeOpt::OnlyCreate => {
            with_create(tcs);
        }
    };
}

fn with_create(tcs: &mut TableCreateStatement) -> &mut TableCreateStatement {
    tcs.col(ColumnDef::new(TimeCol::CreateAt).timestamp())
        .col(ColumnDef::new(TimeCol::CreateBy).integer())
}

fn with_update(tcs: &mut TableCreateStatement) -> &mut TableCreateStatement {
    tcs.col(ColumnDef::new(TimeCol::UpdateAt).timestamp())
        .col(ColumnDef::new(TimeCol::UpdateBy).integer())
}

fn with_str(tcs: &mut TableCreateStatement) -> &mut TableCreateStatement {
    tcs.col(ColumnDef::new(TimeCol::CreateByStr).timestamp())
        .col(ColumnDef::new(TimeCol::UpdateByStr).integer())
}
