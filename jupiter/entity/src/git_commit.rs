//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "git_commit")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub repo_id: i64,
    pub commit_id: String,
    pub tree: String,
    pub parents_id: Vec<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub author: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub committer: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub content: Option<String>,
    pub size: i32,
    #[sea_orm(column_type = "Text")]
    pub full_path: String,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
