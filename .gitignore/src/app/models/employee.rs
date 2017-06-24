#[derive(RustcEncodable, RustcDecodable)]
pub struct User {
	id : i64,
	name : String,
	listsample : Vec<String>
}

impl Employee {
	pub fn get_by_id(id : i64) -> User {
		let employee = Employee {
			id : id,
			age: age,
			name : "yaoguang".to_string(),
		};
		employee
	}
}