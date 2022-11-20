//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.3
use async_graphql::{SimpleObject, ComplexObject, Context};


use sea_orm::entity::prelude::*;

use crate::chef;


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq,SimpleObject)]
#[graphql(complex, name = "Bakery")]
#[sea_orm(table_name = "bakery")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub profit_margin: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::chef::Entity")]
    Chef,
}

impl Related<super::chef::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Chef.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[ComplexObject]
impl  Model {
    async fn chefs(&self,ctx:&Context<'_>) -> Result<Vec<chef::Model>,DbErr>{
        let db = ctx.data::<DatabaseConnection>().unwrap();
        self.find_related(chef::Entity).all(db).await
    }
}