use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::result::Result;
use std::process::Command;

fn save_upload_to_tmp_file(image_stream: &mut Read) -> Result<String, &'static str> {
    let mut image = Vec::new();
    // read the whole file
    if image_stream.read_to_end(&mut image).is_err() {
        return Err("could not read input file into buffer");
    }
    println!("read {} bytes", image.len());
    let file_path = format!("/tmp/image.jpg");
    // TODO: use random name to prevent collisions
    let mut tmp_file = match File::create(&file_path) {
        Ok(f) => f,
        Err(_) => return Err("could not create temporary file"),
    };
    if tmp_file.write_all(image.as_slice()).is_err() {
        return Err("could not write to temporary file");
    }
    //tmp_file.flush().expect("could not flush file");

    Ok(file_path)
}

pub fn process_image(image_stream: &mut Read) -> Result<String, &'static str> {
    println!("start processing a new image …");

    let file_path = save_upload_to_tmp_file(image_stream)?;

    // TODO: specify script location via .env like
    // let script = env::var("SCRIPT_LOCATION").expect("SCRIPT_LOCATION must be set");
    println!("--image_file {}", file_path);
    let out = match Command::new("python")
              .arg("tensorflow_models/tutorials/image/imagenet/classify_image.py")
              .arg("--image_file")
              .arg(file_path)
              .output() {
        Ok(out) => out,
        Err(_) => return Err("error running script"),
    };

    let result = String::from_utf8_lossy(&out.stdout).into_owned();

    println!("{}", result);

    println!("finished processing an image.");
    Ok(result)
}
