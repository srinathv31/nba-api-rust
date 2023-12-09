use std::{fs, env, path::Path};
use serde_json::{Value, Error};

pub fn get_json() -> Result<Value, Box<dyn std::error::Error>> {
    // Grab JSON file
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR")?;

    let file_path = Path::new(&cargo_manifest_dir).join("allTeamData.json");

    println!("{}", file_path.display());

    let contents_result = fs::read_to_string(file_path);

    let contents = match contents_result {
        Ok(contents) => contents,
        Err(error) => return Err(Box::new(error)),
    };

    let data: Result<Value, Error> = serde_json::from_str(&contents);
    return Ok(data?);
}
