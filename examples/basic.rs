#![feature(await_macro, futures_api, async_await)]

use actix_web::{http, server, App, Path, Responder, Result};
use actix_web_async_await::{await, compat};
use std::time::{Instant, Duration};
use tokio::timer::Delay;

async fn index(info: Path<(u32, String)>) -> Result<impl Responder> {
    // Wait 2s
    await!(Delay::new(Instant::now() + Duration::from_secs(2)))?;

    // Proceed with normal response
    Ok(format!("Hello {}! id:{}", info.1, info.0))
}

fn main() {
    server::new(
        || App::new()
            .route("/{id}/{name}/index.html", http::Method::GET, compat(index)))
        .bind("127.0.0.1:8080").unwrap()
        .run();
}
