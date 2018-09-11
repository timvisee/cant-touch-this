use std::collections::HashMap;
use rocket;
use rocket_contrib::Template;

pub struct Server {}

impl Server {
    pub fn new() -> Server {
        Server {}
    }

    pub fn start(&self) {
        rocket::ignite()
            .mount("/", routes![index, example])
            .attach(Template::fairing())
            .launch();
    }
}

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("name", "world");

    Template::render("index", &context)
}

#[get("/<name>")]
fn example(name: String) -> Template {
    let mut context = HashMap::new();
    context.insert("name", name);

    Template::render("index", &context)
}
