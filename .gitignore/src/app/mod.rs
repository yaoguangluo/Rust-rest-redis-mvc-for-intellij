extern crate yaoguang;

mod routes;
mod controllers;
mod models;


use app::routes::*;
use self::yaoguang::prelude::Yaoguang;

pub fn run() {
    Yaoguang::new(routes::all()).http("localhost:8080").unwrap();
}
