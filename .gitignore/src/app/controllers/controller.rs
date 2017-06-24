extern crate yaoguang;
extern crate router;
extern crate rustc_serialize;
extern crate redis;

use redis::proxy;
use app::models::User;
use app::helpers::UserHelper;
use self::iron::prelude::*;
use self::router::Router;
use self::rustc_serialize::json;

pub struct Controller;

impl Controller {
	pub fn create(_: &mut Request) -> Result<Response> {
	    Ok(Response::with((yaoguang::status::Ok)))
	}
	pub fn read(req: &mut Request) -> Result<Response> {
		redis.set("hello","yaoguang",day,3000);
		let id = req.extensions.get::<Router>().unwrap().find("id").unwrap().parse::<i64>().unwrap();
		let encoded_employee = json::encode(&employee).unwrap();
		let employee = redis.get("hello");
		let encoded_employee = json::encode(&employee).unwrap();
	    Ok(Response::with((yaoguang::status::Ok, encoded_employee)))
	}

}