use actix_web::{
    AsyncResponder, Error, FromRequest, HttpMessage, HttpRequest, HttpResponse, Query,
};
use futures::Future;

use crate::{
    state::{GetIPV6, UpdateIPV6},
    State,
};

macro_rules! server_err {
    ($msg:expr) => {
        Err(actix_web::error::ErrorInternalServerError($msg))
    };
}

type ResponseFeature = Box<dyn Future<Item = HttpResponse, Error = Error>>;

pub fn index(_req: HttpRequest<State>) -> HttpResponse {
    HttpResponse::from("Welcome to DDNS API server")
}

pub fn get(req: HttpRequest<State>) -> ResponseFeature {
    let params: Query<GetIPV6> = Query::extract(&req).unwrap();
    let state = req.state().get();
    state
        .send(GetIPV6 {
            name: params.name.clone(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().body(res)),
            Err(_) => server_err!("Failed to get ipv6"),
        })
        .responder()
}

pub fn update(req: HttpRequest<State>) -> ResponseFeature {
    req.json()
        .from_err()
        .and_then(move |update_ipv6: UpdateIPV6| {
            let state = req.state().get();
            state.send(update_ipv6).from_err().and_then(|e| match e {
                Ok(_) => Ok(HttpResponse::Ok().finish()),
                Err(_) => server_err!("Failed to update ipv6"),
            })
        })
        .responder()
}
