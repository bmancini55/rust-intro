use futures::AsyncWriteExt;
use futures::StreamExt;
use sqlx::mysql::*;
use sqlx::prelude::*;
use std::env;
use tokio::fs::File;
use tokio::prelude::*;

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

    // Map into user defined type
    let mut stream = sqlx::query("select unix from candle.binance_btc_usdt")
        .map(|row: sqlx::mysql::MySqlRow| {
            let unix: i32 = row.get(0);
            unix
        })
        .fetch(&pool);

    // We need to use futures::StreamExto use the `next` method
    while let Some(record) = stream.next().await {
        println!("{}", record.unwrap());
    }

    //
    let mut stream =
        sqlx::query_as::<_, Candle>("select unix from candle.binance_btc_usdt").fetch(&pool);
    // We need to use futures::StreamExt to use the `next` method again
    while let Some(candle) = stream.next().await {
        println!("woot {}", candle?.unix);
    }

    // construct a file and we will stream shit into it!
    let mut file = File::create("foo.txt").await?;
    let mut stream =
        sqlx::query_as::<_, Candle>("select unix from candle.binance_btc_usdt").fetch(&pool);
    while let Some(candle) = stream.next().await {
        let string: String = format!("wrote {}\n", candle?.unix);
        file.write(string.as_bytes()).await?;
    }

    Ok(())
}
