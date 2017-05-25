use std::result::Result;
use std::process::Command;

pub fn process_image() -> Result<String, &'static str> {
    println!("start processing a new image …");

    // TODO: specify script location via .env like
    // let script = env::var("SCRIPT_LOCATION").expect("SCRIPT_LOCATION must be set");

    let out = match Command::new("python")
              .arg("tensorflow_models/tutorials/image/imagenet/classify_image.py")
              .output() {
        Ok(out) => out,
        Err(_) => return Err("error running script"),
    };

    let result = String::from_utf8_lossy(&out.stdout).into_owned();

    println!("{}", result);

    println!("finished processing an image.");
    Ok(result)
}
