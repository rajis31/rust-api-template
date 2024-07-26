use std::env;
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    // dotenv().ok();
    let db = env::var("ROCKET_DB").unwrap_or_else(|_| "test".to_string());
    println!("{}", db);
    rocket::build().mount("/", routes![index])
}