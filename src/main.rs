#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate schani_image_recognition;

use rocket::Data;
use schani_image_recognition::recognition::process_image;

#[post("/recognize", data = "<data>")]
fn recognize(data: Data) -> String {
    match process_image(&mut data.open()) {
        Ok(out) => out,
        Err(err) => err.to_string(),
    }
}

fn main() {
    rocket::ignite().mount("/", routes![recognize]).launch();
}
