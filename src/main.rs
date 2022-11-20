mod errorhandle;
mod router;
mod schema;

use rocket::{ launch, routes, State, catchers, response::content};
use async_graphql::{EmptySubscription, Schema, http::{playground_source, GraphQLPlaygroundConfig}};
use async_graphql_rocket::*;


use schema::*;

type SchemaType = Schema<QueryRoot,MutationRoot,EmptySubscription>;


#[rocket::post("/graphql",data="<req>",format="application/json")]
async fn graphql_req(schema: &State<SchemaType>,req:GraphQLRequest)-> GraphQLResponse{
    req.execute(schema).await
}

#[rocket::get("/graphql")]
fn graphql_playground() -> content::RawHtml<String> {
  content::RawHtml(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}



#[launch]
async fn rocket()  -> _ {
    let db = match dbbackend::establish_connection().await {
        Ok(db) => db,
        Err(err) => panic!("err :{}", err)
    };

    // build the schema
    let schema = Schema::build(QueryRoot,MutationRoot,EmptySubscription).data(db.clone()).finish();

    rocket::build()
    .manage(db)
    .manage(schema)
    .mount("/",routes![
        router::hello,
        router::bakeries,
        router::fetch_one,
        router::new_bakery,
        graphql_req
    ])
    .mount("/",routes![graphql_playground])
    .register("/",catchers![])
}