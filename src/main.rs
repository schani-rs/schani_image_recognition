#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate schani_image_recognition;

use schani_image_recognition::recognition::process_image;

#[post("/recognize")]
fn recognize() -> String {
    match process_image() {
        Ok(out) => out,
        Err(err) => err.to_string()
    }
}

fn main() {
    rocket::ignite().mount("/", routes![recognize]).launch();
}
