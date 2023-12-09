use std::{fs, path::Path};
use serde_json::{Value, Error};

pub fn get_json() -> Result<Value, Box<dyn std::error::Error>> {
    // Grab JSON file
    println!("Current working directory: {:?}", std::env::current_dir());
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
    let file_path = Path::new("allTeamData.json").canonicalize()?.to_str().unwrap().to_owned();
    let contents_result = fs::read_to_string(file_path);

    let contents = match contents_result {
        Ok(contents) => contents,
        Err(error) => return Err(Box::new(error)),
    };

    let data: Result<Value, Error> = serde_json::from_str(&contents);
    return Ok(data?);
}
