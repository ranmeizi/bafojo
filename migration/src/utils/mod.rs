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
    tcs.col(ColumnDef::new(TimeCol::CreatedAt).timestamp())
        .col(ColumnDef::new(TimeCol::CreatedBy).integer())
}

fn with_update(tcs: &mut TableCreateStatement) -> &mut TableCreateStatement {
    tcs.col(ColumnDef::new(TimeCol::UpdatedAt).timestamp())
        .col(ColumnDef::new(TimeCol::UpdatedBy).integer())
}

fn with_str(tcs: &mut TableCreateStatement) -> &mut TableCreateStatement {
    tcs.col(ColumnDef::new(TimeCol::CreatedByStr).timestamp())
        .col(ColumnDef::new(TimeCol::UpdatedByStr).integer())
}
