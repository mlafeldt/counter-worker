use std::sync::Mutex;

use worker::*;

lazy_static::lazy_static! {
    static ref COUNTER: Mutex<usize> = Mutex::new(0);
}

#[event(fetch)]
pub async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let mut counter = COUNTER.lock().unwrap();
    *counter += 1;

    console_log!(
        "counter={} cf={:?} headers={:?}",
        *counter,
        req.cf(),
        req.headers()
    );

    Response::ok(format!(
        "Counter: {}\nCould start: {}\nDatacenter: {}\n",
        *counter,
        *counter == 1,
        req.cf().colo()
    ))
}
