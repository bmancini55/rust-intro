use hyper::body::Bytes;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::time::{SystemTime, UNIX_EPOCH};

use futures::StreamExt;
use sqlx::mysql::*;
use sqlx::prelude::*;

use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // obtain the database url
    let database_url = &env::var("DATABASE_URL").unwrap();

    // Create a connection pool
    let pool = MySqlPool::new(database_url).await.unwrap();

    // A `Service` is needed for every connection, so this
    // creates one from our `handle_conn` function.
    let make_svc = make_service_fn(move |_conn| {
        let pool = pool.clone();

        async move {
            // service_fn converts our function into a `Service`
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let pool = pool.clone();
                handle_request(req, pool)
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn handle_request(req: Request<Body>, pool: MySqlPool) -> Result<Response<Body>, Infallible> {
    middleware_log(&req).await;

    match (req.method(), req.uri().path()) {
        // index
        (&Method::GET, "/") => hello_world(req).await,

        // inside that match from before
        (&Method::GET, "/echo") => handle_echo(req).await,

        // simple async method
        (&Method::GET, "/async") => run_async(req).await,

        (&Method::GET, "/data") => run_data(req, pool).await,

        // 404
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            *not_found.body_mut() = Body::from("Not Found");
            Ok(not_found)
        }
    }
}

async fn middleware_log(req: &Request<Body>) {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let method = req.method();
    let path_and_query = req.uri().path_and_query().unwrap();
    println!(
        "{} {}: {}",
        since_the_epoch.as_millis(),
        method,
        path_and_query
    )
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

async fn handle_echo(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(req.into_body()))
}

async fn run_async(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let (mut channel, body) = Body::channel();

    // if we don't spawn a task then the sender will block after the
    // first message is sent!
    tokio::spawn(async move {
        for _ in 1..10 {
            channel
                .send_data(Bytes::from(&b"Hello world\n"[..]))
                .await
                .unwrap();
        }
    });
    Ok(Response::new(body))
}

async fn run_data(_req: Request<Body>, pool: MySqlPool) -> Result<Response<Body>, Infallible> {
    let (mut channel, body) = Body::channel();

    // Spawn a new worker task which will execute our query task
    // and will write to the producer channel!
    tokio::spawn(async move {
        // Map into user defined type
        let mut stream = sqlx::query("select unix from candle.binance_btc_usdt")
            .map(|row: sqlx::mysql::MySqlRow| {
                let unix: i32 = row.get(0);
                unix
            })
            .fetch(&pool);

        // We need to use futures::StreamExto use the `next` method
        let mut count: i32 = 0;
        while let Some(record) = stream.next().await {
            let data = format!("{}\n", record.unwrap());
            channel.send_data(data.into()).await.unwrap();
            std::thread::sleep(std::time::Duration::from_millis(1));
            count += 1;
            println!("row {}", count);
        }
    });

    Ok(Response::new(body))
}
