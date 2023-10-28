mod router;
mod state;

use std::env;
use actix::System;
use actix_web::{App, middleware::Logger, http, server};
use log::{info, error};
use router::{get, index, update};
use state::State;

fn env_init() {
    env::set_var("RUST_LOG", "ddns=info");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
}

fn main() {
    env_init();
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        error!("need a path");
        return ;
    }
    let path = &args[1];
    let port = &args[2];
    let addr = "0.0.0.0:".to_owned() + &port;
    let system = System::new("ddns");
    let state = State::init(path.to_owned() + "/ddns.bin");

    let web_app = move || {
        App::with_state(state.clone())
        .middleware(Logger::default())
        .route("/", http::Method::GET, index)
        .route("/ipv6", http::Method::GET, get)
        .route("/ipv6", http::Method::POST, update)
    };

    info!("Start http server: {}", addr);
    server::new(web_app).bind(addr).unwrap().start();
    let _ =  system.run();
}
