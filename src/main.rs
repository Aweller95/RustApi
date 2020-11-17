#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_attributes)]

use rocket::*;
use rocket_contrib::helmet::SpaceHelmet;
use rocket_contrib::serve::StaticFiles;

mod routes;

pub fn rocket_builder() -> rocket::Rocket {
    rocket::ignite()
        .attach(SpaceHelmet::default())
        .mount("/", routes![routes::echo::echo_fn])
        .mount("/files", StaticFiles::from("static/"))
}

fn main() {
    rocket_builder().launch();
}

//https://dev.to/davidedelpapa/rocket-tutorial-02-minimalist-api-2kl6
