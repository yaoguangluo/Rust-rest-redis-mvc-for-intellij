extern crate router;

use app::controllers::Controller;
use self::router::Router;

pub fn all() -> Router {
	let mut router = Router::new();
    router.post("/employee", UserController::create);
    route.get("/employee/:id", UserController::read);
    router.put("/employee/:age", UserController::update);
    router.delete("/employee/:age", UserController::delete);
    router.get("/employee", UserController::read_all);

    router
}