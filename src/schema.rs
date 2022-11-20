use async_graphql::{Context, Object};
use entity::{bakery, prelude::{Bakery, Chef}, chef};
use migration::DbErr;
use sea_orm::{DatabaseConnection, EntityTrait, ActiveValue};

pub struct QueryRoot;
pub struct MutationRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> String {
        "hello graphql".to_owned()
    }

    // For finding all bakeries
    async fn bakeries(&self, ctx: &Context<'_>) -> Result<Vec<bakery::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Bakery::find().all(db).await
    }

    // For finding one bakery by id
    async fn bakery(&self, ctx: &Context<'_>, id: i32) -> Result<Option<bakery::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        Bakery::find_by_id(id).one(db).await
    }
}


#[Object]
impl MutationRoot {

    // for inserting a bakery
    async fn add_bakery(&self,ctx:&Context<'_>,name:String) -> Result<bakery::Model,DbErr>{
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let res = Bakery::insert(bakery::ActiveModel{
            name:ActiveValue::Set(name),
            profit_margin:ActiveValue::Set(0.0.to_string()),
            ..Default::default()
        }).exec(db).await?;

        Bakery::find_by_id(res.last_insert_id).one(db).await.map(|b|b.unwrap())
    }
    

    // for inserting a chef
    async fn add_chef(&self,ctx: &Context<'_>,name:String,bakery_id:i32)->Result<chef::Model,DbErr>{
        let db = ctx.data::<DatabaseConnection>().unwrap();
        
        let res = Chef::insert(chef::ActiveModel{
            name:ActiveValue::Set(name),
            bakery_id:ActiveValue::Set(bakery_id),
            ..Default::default()
        }).exec(db).await?;

        Chef::find_by_id(res.last_insert_id).one(db).await.map(|c|c.unwrap())
    }

}