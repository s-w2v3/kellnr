//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "krate")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(column_type = "Text", unique)]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub max_version: String,
    pub total_downloads: i64,
    #[sea_orm(column_type = "Text")]
    pub last_updated: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub homepage: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub repository: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub original_name: String,
    pub e_tag: String,
    pub restricted_download: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::crate_author_to_crate::Entity")]
    CrateAuthorToCrate,
    #[sea_orm(has_many = "super::crate_category_to_crate::Entity")]
    CrateCategoryToCrate,
    #[sea_orm(has_many = "super::crate_group::Entity")]
    CrateGroup,
    #[sea_orm(has_many = "super::crate_index::Entity")]
    CrateIndex,
    #[sea_orm(has_many = "super::crate_keyword_to_crate::Entity")]
    CrateKeywordToCrate,
    #[sea_orm(has_many = "super::crate_meta::Entity")]
    CrateMeta,
    #[sea_orm(has_many = "super::crate_user::Entity")]
    CrateUser,
    #[sea_orm(has_many = "super::owner::Entity")]
    Owner,
}

impl Related<super::crate_author_to_crate::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CrateAuthorToCrate.def()
    }
}

impl Related<super::crate_category_to_crate::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CrateCategoryToCrate.def()
    }
}

impl Related<super::crate_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CrateGroup.def()
    }
}

impl Related<super::crate_index::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CrateIndex.def()
    }
}

impl Related<super::crate_keyword_to_crate::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CrateKeywordToCrate.def()
    }
}

impl Related<super::crate_meta::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CrateMeta.def()
    }
}

impl Related<super::crate_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CrateUser.def()
    }
}

impl Related<super::owner::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Owner.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
