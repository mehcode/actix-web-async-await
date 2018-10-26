#![feature(await_macro, futures_api, async_await)]

use std::future::Future;
use tokio_async_await::compat::backward::Compat;
use actix_web::FutureResponse;

// Re-export `tokio::await` for ease-of-use
pub use tokio_async_await::await;

macro_rules! define_compat {
    ($name:ident($($arg:ident),+: $($ty:ident),+)) => (
        #[inline]
        pub fn $name<F, Fut, Ret, Err, $($ty,)*>(f: F) -> impl Fn($($ty,)*) -> FutureResponse<Ret, Err>
        where
            F: Fn($($ty,)*) -> Fut,
            Fut: Future<Output = Result<Ret, Err>> + 'static,
        {
            move |$($arg,)*| Box::new(Compat::new(f($($arg,)*)))
        }
    );
}

define_compat!(compat(arg1: Arg1));
define_compat!(compat2(arg1, arg2: Arg1, Arg2));
define_compat!(compat3(arg1, arg2, arg3: Arg1, Arg2, Arg3));
define_compat!(compat4(arg1, arg2, arg3, arg4: Arg1, Arg2, Arg3, Arg4));
define_compat!(compat5(arg1, arg2, arg3, arg4, arg5: Arg1, Arg2, Arg3, Arg4, Arg5));
define_compat!(compat6(arg1, arg2, arg3, arg4, arg5, arg6: Arg1, Arg2, Arg3, Arg4, Arg5, Arg6));
