#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate schani_image_recognition;

use rocket::Data;
use rocket_contrib::JSON;
use schani_image_recognition::recognition::{Prediction, process_image};

#[post("/recognize", data = "<data>")]
fn recognize(data: Data) -> Result<JSON<Vec<Prediction>>, String> {
    Ok(JSON(try!(process_image(&mut data.open()).map_err(
        |err| err.to_string(),
    ))))
}

fn main() {
    rocket::ignite().mount("/", routes![recognize]).launch();
}
