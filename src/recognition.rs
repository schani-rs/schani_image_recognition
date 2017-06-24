use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;
use std::result::Result;
use std::vec::Vec;

use serde_json;
use temporary::Directory;

#[derive(Debug, Deserialize, Serialize)]
pub struct Prediction {
    class: String,
    score: f64,
}

fn save_upload_to_tmp_file(
    tmp_dir: &Directory,
    image_stream: &mut Read,
) -> Result<PathBuf, &'static str> {
    let mut image = Vec::new();
    // read the whole file

    try!(image_stream.read_to_end(&mut image).map_err(
        |_| "could not read input file into buffer",
    ));
    info!("read {} bytes", image.len());
    let file_path = tmp_dir.join("image.jpg");
    let mut tmp_file = try!(File::create(&file_path).map_err(
        |_| "could not create temporary file",
    ));
    try!(tmp_file.write_all(image.as_slice()).map_err(
        |_| "could not write to temporary file",
    ));
    try!(tmp_file.flush().map_err(
        |_| "could not flush temporary file",
    ));

    Ok(file_path)
}

pub fn process_image(image_stream: &mut Read) -> Result<Vec<Prediction>, String> {
    info!("start processing a new image …");

    let tmp_dir = try!(Directory::new("image").map_err(|err| {
        format!("could not create temp dir: {}", err)
    }));
    let file_path = save_upload_to_tmp_file(&tmp_dir, image_stream)?;

    // TODO: specify script location via .env like
    // let script = env::var("SCRIPT_LOCATION").expect("SCRIPT_LOCATION must be set");
    debug!("--image_file {:?}", file_path);
    let out = try!(
        Command::new("python3")
            .arg(
                "tensorflow_models/tutorials/image/imagenet/classify_image.py",
            )
            .arg("--image_file")
            .arg(file_path)
            .output()
            .map_err(|err| format!("error running script: {}", err))
    );

    let result = String::from_utf8_lossy(&out.stdout).into_owned();
    let error = String::from_utf8_lossy(&out.stderr).into_owned();

    if !out.status.success() {
        return Err(format!(
            "command returned unsuccessful exit status {} with stderr: {}",
            out.status,
            error
        ));
    }

    info!("json: {}", result);
    info!("error: {}", error);
    let predicitons: Vec<Prediction> = try!(serde_json::from_str(&result).map_err(|err| {
        format!("could not parse json: {}", err)
    }));

    info!("finished processing an image: {:?}", predicitons);
    Ok(predicitons)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn test_process_sample1() {
        let mut test_file_stream = File::open("resources/sample1.jpg").unwrap();
        let mut buf_read = BufReader::new(&mut test_file_stream);
        process_image(&mut buf_read).unwrap();
    }

    #[test]
    fn test_process_sample2() {
        let mut test_file_stream = File::open("resources/sample2.jpg").unwrap();
        let mut buf_read = BufReader::new(&mut test_file_stream);
        process_image(&mut buf_read).unwrap();
    }

    #[test]
    fn test_process_sample3() {
        let mut test_file_stream = File::open("resources/sample3.jpg").unwrap();
        let mut buf_read = BufReader::new(&mut test_file_stream);
        process_image(&mut buf_read).unwrap();
    }
}
