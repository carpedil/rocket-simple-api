use rocket::{ launch, routes};


mod errorhandle;
mod router;


#[launch]
async fn rocket()  -> _ {
    let db = match dbbackend::establish_connection().await {
        Ok(db) => db,
        Err(err) => panic!("err :{}", err)
    };

    rocket::build()
    .manage(db)
    .mount("/",routes![
        router::hello,
        router::bakeries,
        router::fetch_one,
        router::new_bakery,
    ])
}