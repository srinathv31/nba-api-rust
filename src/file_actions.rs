use std::{fs, env};
use serde_json::{Value, Error};

pub fn get_json() -> Result<Value, Box<dyn std::error::Error>> {
    // Grab JSON file
    // Get the path to the executable
    let mut exe_path = env::current_exe()?;
    
    // Navigate to the parent directory of the executable
    exe_path.pop();

    // Construct the full path to the JSON file
    let file_path = exe_path.join("allTeamData.json");

    println!("{}", file_path.display());

    let contents_result = fs::read_to_string(file_path);

    let contents = match contents_result {
        Ok(contents) => contents,
        Err(error) => return Err(Box::new(error)),
    };

    let data: Result<Value, Error> = serde_json::from_str(&contents);
    return Ok(data?);
}
