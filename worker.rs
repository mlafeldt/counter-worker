use std::sync::Mutex;

use worker::*;

lazy_static::lazy_static! {
    static ref COUNTER: Mutex<usize> = Mutex::new(0);
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    if req.path() != "/" {
        return Response::error("Not found", 404);
    }

    let ip = req.headers().get("x-real-ip").ok().flatten().unwrap();

    let cache = env.kv("COUNTER_CACHE").unwrap();

    if let Some(counter) = cache.get(&ip).cache_ttl(60).text().await? {
        console_log!("kv_counter={}", counter);
    } else {
        console_log!("kv_counter is empty");
    }

    let mut counter = COUNTER.lock().unwrap();
    *counter += 1;

    let cache = env.kv("COUNTER_CACHE").unwrap();
    // cache
    //     .put(&ip, *counter)
    //     .unwrap()
    //     .expiration_ttl(60)
    //     .execute()
    //     .await?;

    console_log!(
        "counter={} cf={:?} headers={:?}",
        *counter,
        req.cf(),
        req.headers()
    );

    Response::ok(format!(
        "Counter: {}\nCold start: {}\nDatacenter: {}\nPkg version: {}",
        *counter,
        *counter == 1,
        req.cf().colo(),
        env!("CARGO_PKG_VERSION")
    ))
}
