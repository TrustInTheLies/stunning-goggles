//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(column_type = "Text")]
    pub password: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub token: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::tracks::Entity> for Entity {
    fn to() -> RelationDef {
        super::users_tracks::Relation::Tracks.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::users_tracks::Relation::Users.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
