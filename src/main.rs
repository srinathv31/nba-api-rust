use std::fs;

use actix_web::{get, web, Result, Responder};
use serde_json::Value;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new()
        .service(index)
        .service(get_team_year)
        .service(get_roster)
        .service(get_schedule)
        )
            .bind(("127.0.0.1", 8080))?
            .run()
            .await
}

#[get("/")]
async fn index() -> Result<impl Responder> {
    Ok("Hit our endpoints to get NBA Data")
}

#[get("/v1/nba/{team}/{year}")]
async fn get_team_year(path: web::Path<(String, String)>) -> Result<impl Responder> {
    // parse url params
    let (team, year) = path.into_inner();

    // Grab JSON file
    let file_path = "./allTeamData.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
    // let data: NbaData = serde_json::from_str(&contents).unwrap();
    let data: Value = serde_json::from_str(&contents).expect("OP");

    // access relevant data
    let res = &data[&team.to_string()][&year.to_string()];

    Ok(format!("{}", res))
}

#[get("/v1/nba/{team}/{year}/roster")]
async fn get_roster(path: web::Path<(String, String)>) -> Result<impl Responder> {
    // parse url params
    let (team, year) = path.into_inner();

    // Grab JSON file
    let file_path = "./allTeamData.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
    // let data: NbaData = serde_json::from_str(&contents).unwrap();
    let data: Value = serde_json::from_str(&contents).expect("OP");

    // access relevant data
    let res = &data[&team.to_string()][&year.to_string()]["Roster"];

    Ok(format!("{}", res))
}

#[get("/v1/nba/{team}/{year}/schedule")]
async fn get_schedule(path: web::Path<(String, String)>) -> Result<impl Responder> {
    // parse url params
    let (team, year) = path.into_inner();

    // Grab JSON file
    let file_path = "./allTeamData.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
    // let data: NbaData = serde_json::from_str(&contents).unwrap();
    let data: Value = serde_json::from_str(&contents).expect("OP");

    // access relevant data
    let res = &data[&team.to_string()][&year.to_string()]["Schedule"];

    Ok(format!("{}", res))
}

