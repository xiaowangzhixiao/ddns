mod router;
mod state;

use std::env;
use actix::System;
use actix_web::{App, middleware::Logger, http, server};
use log::info;
use router::{get, index, update};
use state::State;

fn env_init() {
    env::set_var("RUST_LOG", "ddns=info");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    info!("Start http server: 127.0.0.1:8080");
}

fn main() {
    env_init();
    let system = System::new("ddns");
    let state = State::init("./ddns.bin".to_string());

    let web_app = move || {
        App::with_state(state.clone())
        .middleware(Logger::default())
        .route("/", http::Method::GET, index)
        .route("/ipv6", http::Method::GET, get)
        .route("/ipv6", http::Method::POST, update)
    };

    server::new(web_app).bind("127.0.0.1:8080").unwrap().start();
    let _ =  system.run();
}
