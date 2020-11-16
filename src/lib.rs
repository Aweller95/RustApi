#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_attributes)]

#[macro_use]
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
