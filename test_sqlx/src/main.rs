use sqlx::mysql::*;
use sqlx::prelude::*;
use std::env;

#[derive(sqlx::FromRow)]
struct Candle {
    unix: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = &env::var("DATABASE_URL")?;

    // Create a connection pool
    let pool = MySqlPool::new(database_url).await?;

    // Fetch a single row
    let row: (i32,) = sqlx::query_as("select unix from candle.binance_btc_usdt")
        .fetch_one(&pool)
        .await?;
    println!("{:?}", row);

    // Stream some results using fetch
    let mut stream = sqlx::query("select unix from candle.binance_btc_usdt").fetch(&pool);
    while let Some(row) = stream.next().await? {
        let unix: i32 = row.get(0);
        println!("{}", unix);
    }

    // AHHHHHHH
    // let mut stream =
    //     sqlx::query_as::<_, Candle>("select unix from candle.binance_btc_usdt").fetch(&pool);
    // for candle in stream.next().await? {
    //     println!("{}", candle.unix);
    // }

    Ok(())
}
