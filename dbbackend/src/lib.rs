use sea_orm::*;

use std::env;
use dotenv::dotenv;

// const DATABASE_URL :&str = "postgres://postgres:root@localhost:5432/postgres";
// const DB_NAME:&str = "bakeries_db";


pub async fn establish_connection() ->Result<DatabaseConnection,DbErr>{
    dotenv().ok();
    let db = Database::connect(env::var("DATABASE_URL").unwrap()).await?;
    Ok(db)
}

#[macro_use]
#[cfg(test)]
mod tests {
    
    use super::establish_connection;

    // 使用宏来调用async函数
    macro_rules! async_test{
        ($e:expr) => {
            tokio_test::block_on($e).unwrap()
        }
    }

    async fn get_db() -> Result<(),()>{
        let _db =  establish_connection().await.unwrap();
        println!("db: {:?}", _db);
        Ok(())
    }

    #[test]
    fn test_get_db() {
        async_test!(get_db())
    }
}