#![deny(warnings)]
#[macro_use]
extern crate warp;

#[macro_use]
extern crate log;
extern crate log4rs;

use warp::{Filter, Rejection};

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let hello2 = path!("hello2").map(|| { "hello2" });

    let routes = hello_world_endpoint().or(hello2);

    let logged_routes = routes.with(warp::log("access_log"));


    info!("starting server.");
    warp::serve(logged_routes)
        .run(([127, 0, 0, 1], 3030));
}

#[inline]
fn hello_world_endpoint() -> impl Filter<Extract = (String, ), Error = Rejection> + Copy {
    hello_world_filter().map(|s: String, i: i32, j: i32| {
        format!("Hello {}, {}, {}", s, i, j)
    })
}

#[inline]
fn hello_world_filter() -> impl Filter<Extract = (String, i32, i32), Error = Rejection> + Copy {
    path!("hello" / String / i32 / i32)
}