#[macro_use] extern crate rocket;

mod api;

// TODO: get this from a config, also figure out how this works lol
const HOST: rocket::http::uri::Absolute<'static> = uri!("http://localhost:8000");

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(api::stage())
}
