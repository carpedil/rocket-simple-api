use entity::bakery;
use entity::prelude::*;
use rocket::{serde::json::Json, get, State, post};
use sea_orm::ActiveValue;
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::errorhandle::ErrorResponder;



#[get("/")]
pub async fn hello() -> &'static str {    
    "Hello bakeries !!"
}


#[get("/bakeries")]
pub async fn bakeries(db: &State<DatabaseConnection>) -> Result<Json<Vec<String>>,ErrorResponder>{
    let db = db as &DatabaseConnection;

    let bakery_names = Bakery::find()
    .all(db)
    .await
    .map_err(Into::<ErrorResponder>::into)?
    .into_iter()
    .map(|b|b.name)
    .collect::<Vec<String>>();

    Ok(Json(bakery_names))
}


#[get("/bakeries/<id>")]
pub async fn fetch_one(db: &State<DatabaseConnection>,id:i32) -> Result<String,ErrorResponder>{
    let db = db as &DatabaseConnection;

    let bakery = Bakery::find_by_id(id).one(db).await.map_err(Into::<ErrorResponder>::into)?;

    Ok(if let Some(bakery) = bakery {
        bakery.name
    } else {
        return Err(format!("No bakery with id {id} is found.").into());
    })
}

#[post("/bakeries?<name>&<profit_margin>")]
pub async fn new_bakery(
    db: &State<DatabaseConnection>,
    name: &str,
    profit_margin: Option<f64>,
) -> Result<(), ErrorResponder> {
    let db = db as &DatabaseConnection;

    let new_bakery = bakery::ActiveModel {
        name: ActiveValue::Set(name.to_owned()),
        profit_margin: ActiveValue::Set(profit_margin.unwrap_or_default().to_string()),
        ..Default::default()
    };

    Bakery::insert(new_bakery)
        .exec(db)
        .await
        .map_err(Into::<ErrorResponder>::into)?;

    Ok(())
}