#![feature(generators, generator_trait)]

use mysql::prelude::*;
use mysql::*;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::SyncSender;
use std::thread;
use toml::Value;

#[derive(Debug)] // enables the ability to use
pub struct Candle {
    period: i32,
    unix: i32,
    high: f64,
    low: f64,
    open: f64,
    close: f64,
    volume: f64,
    quote_volume: f64,
}

/**
 * Example of adding display to a Candle which can be used with code that looks
 * like:
 *
 * ```rust
 * println!("{0}", selected_candles[0]);
 * ```
 */
impl fmt::Display for Candle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {}, {}, {}, {}, {})",
            self.period,
            self.unix,
            self.open,
            self.high,
            self.low,
            self.close,
            self.volume,
            self.quote_volume
        )
    }
}

// use of ? operator for function calls requires us to use -> Result<()>
fn main() -> Result<()> {
    // open the config file and read the contents
    let mut f = File::open("./config.toml")?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    println!("{}", buffer);

    // parse the toml file and obtain the url
    let value = buffer.parse::<Value>().unwrap();
    let url = value["url"].as_str().unwrap();
    println!("{}", url);

    // construct a pool
    let pool = Pool::new(url)?;

    // example 1: buffer all result to memory
    // obtain a connection from the pool, select the results from the database
    // and into memory and then does some stuff with the result
    let selected_candles = load_candles_direct(&pool)?;
    println!("records read: {0}", selected_candles.len());
    println!("debug:   {:?}", selected_candles[0]); // print via debug
    println!("display: {0}", selected_candles[0]); // print via display

    // example 2: use a channel to stream results
    // slightly more complex example where we spawn a thread to produce results
    // from the database that can be consumed by the main thread using the
    // receiving iterator
    let (sender, receiver) = sync_channel(0);
    let child = thread::spawn({
        let pool = pool.clone();
        move || {
            println!("producer thread has started");
            let result = stream_candles(&pool, &sender);
            println!("producer thread has ended");
            result
        }
    });
    // consume records through a blocking iterator
    receiver.iter().for_each(|candle| println!("{}", candle));
    // capture any errors that happened in the reading thread
    child.join().unwrap()?;

    Ok(())
}

fn load_candles_direct(pool: &mysql::Pool) -> Result<Vec<Candle>> {
    let mut conn = pool.get_conn()?;
    conn.query_map(
        "select * from candle.binance_btc_usdt",
        |(period, unix, high, low, open, close, volume, quote_volume)| crate::Candle {
            period,
            unix,
            high,
            low,
            open,
            close,
            volume,
            quote_volume,
        },
    )
}

fn stream_candles(pool: &mysql::Pool, sender: &SyncSender<Candle>) -> Result<()> {
    let mut conn = pool.get_conn()?;
    let mut result = conn.query_iter("select * from candle.binance_btc_usdt")?;

    let result_set = result.next_set();
    for row in result_set.unwrap().unwrap() {
        let candle = parse_candle(&row?);
        sender.send(candle).unwrap();
    }
    Ok(())
}

fn parse_candle(row: &Row) -> Candle {
    Candle {
        period: row.get(0).unwrap_or_default(),
        unix: row.get(1).unwrap_or_default(),
        high: row.get(2).unwrap_or_default(),
        low: row.get(3).unwrap_or_default(),
        open: row.get(4).unwrap_or_default(),
        close: row.get(5).unwrap_or_default(),
        volume: row.get(6).unwrap_or_default(),
        quote_volume: row.get(7).unwrap_or_default(),
    }
}
