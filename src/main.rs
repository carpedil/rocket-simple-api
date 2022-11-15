use entity::prelude::*;
use rocket::{serde::json::Json, launch, routes, get, State};
use sea_orm::{DatabaseConnection, EntityTrait};


#[get("/")]
async fn index() -> &'static str {    
    "Hello bakeries !!"
}


#[get("/bakeries")]
async fn bakeries(db: &State<DatabaseConnection>) -> Json<Vec<String>>{
    let db = db as &DatabaseConnection;

    let bakery_names = Bakery::find()
    .all(db)
    .await
    .unwrap()
    .into_iter()
    .map(|b|b.name)
    .collect::<Vec<String>>();

    Json(bakery_names)
}


#[launch]
async fn rocket()  -> _ {
    let db = match dbbackend::establish_connection().await {
        Ok(db) => db,
        Err(err) => panic!("err :{}", err)
    };

    rocket::build()
    .manage(db)
    .mount("/",routes![
        index,
        bakeries
    ])
}