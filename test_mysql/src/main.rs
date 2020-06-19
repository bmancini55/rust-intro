use mysql::prelude::*;
use mysql::*;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
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
    let mut conn = pool.get_conn()?;

    // perform the selection
    let selected_candles = conn.query_map(
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
    )?;

    println!("records read: {0}", selected_candles.len());
    println!("debug:   {:?}", selected_candles[0]); // print via debug
    println!("display: {0}", selected_candles[0]); // print via display
    return Ok(());
}
