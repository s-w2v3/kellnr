//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "crate_keyword")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(column_type = "Text", unique)]
    pub keyword: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::crate_keyword_to_crate::Entity")]
    CrateKeywordToCrate,
}

impl Related<super::crate_keyword_to_crate::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CrateKeywordToCrate.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
