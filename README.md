# Actix Web Async/Await Preview
> This crate provides a preview of Actix with async/await support.

## Usage
To use this crate, you need to start with a Rust 2018 edition crate.

Add this to your Cargo.toml:

```toml
# In the `[package]` section
edition = "2018"

# In the `[dependencies]` section
actix-web-async-await = "0.1.0"
```

Then, get started. Here is the [headline Actix example](https://github.com/actix/actix-web#example) with the addition that it asynchronously delays the request by 2 seconds.

The general idea is to wrap your `async fn` handlers in `compat`. There are `compat2`, `compat3`, etc. for routes taking multiple arguments.

```rust
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

```

Note that your `async fn` handlers must return `Result` currently. This is both because they are being converted to `futures v0.1` which requires an error type _and_ because nearly the entire rust async ecosystem uses `futures v0.1` which is going to have errors on all futures. An error-less `compat` could be provided if the support is wanted.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
