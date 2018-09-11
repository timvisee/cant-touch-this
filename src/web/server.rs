use rocket;

pub struct Server {}

impl Server {
    pub fn new() -> Server {
        Server {}
    }

    pub fn start(&self) {
        rocket::ignite().mount("/", routes![index]).launch();
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
