use rocket::{get, launch, routes};
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![root])
}
#[get("/hello")]
async fn root() -> String {
    "Hello, World".to_string()
}