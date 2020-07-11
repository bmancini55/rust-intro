use hyper::body::Bytes;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::time::{SystemTime, UNIX_EPOCH};

use std::convert::Infallible;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `handle_conn` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    middleware_log(&req).await;

    match (req.method(), req.uri().path()) {
        // index
        (&Method::GET, "/") => hello_world(req).await,

        // inside that match from before
        (&Method::GET, "/echo") => handle_echo(req).await,

        // simple async method
        (&Method::GET, "/async") => run_async(req).await,

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
    let (channel, body) = Body::channel();
    let mut channel = channel;
    channel
        .send_data(Bytes::from(&b"Hello world"[..]))
        .await
        .unwrap();
    Ok(Response::new(body))
}
