use std::result::Result;

pub fn process_image() -> Result<(), &'static str> {
    println!("start processing a new image …");

    println!("finished processing an image.");
    Ok(())
}