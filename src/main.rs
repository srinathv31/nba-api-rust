use actix_web::{get, web, Result, Responder, HttpResponse};

mod file_actions;

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

    // Grab JSON data
    let data_result = file_actions::get_json();

    // Parse out data/error
    let data = match data_result {
        Ok(data) => data,
        Err(error) => {
            println!("ERROR: {:?}", error);
            return Ok(HttpResponse::InternalServerError().body("API Error"))},
    };

    // access relevant data
    let res = &data[&team][&year];

    if res.is_null() {
        println!("{}", format!("Team Data was null for {} - {}", year, team));
        let res_msg = format!("Team data for the {} - {} was not found.", year, team);
        return Ok(HttpResponse::NotFound().body(res_msg));
    }

    Ok(HttpResponse::Ok().body(res.to_string()))
}

#[get("/v1/nba/{team}/{year}/roster")]
async fn get_roster(path: web::Path<(String, String)>) -> Result<impl Responder> {
    // parse url params
    let (team, year) = path.into_inner();

    // Grab JSON data
    let data_result = file_actions::get_json();

    // Parse out data/error
    let data = match data_result {
        Ok(data) => data,
        Err(error) => {
            println!("ERROR: {:?}", error);
            return Ok(HttpResponse::InternalServerError().body("API Error"))},
    };

    // access relevant data
    let res = &data[&team][&year]["Roster"];

    if res.is_null() {
        println!("{}", format!("Roster Data was null for {} - {}", year, team));
        let res_msg = format!("Roster for the {} - {} was not found.", year, team);
        return Ok(HttpResponse::NotFound().body(res_msg));
    }

    Ok(HttpResponse::Ok().body(res.to_string()))
}

#[get("/v1/nba/{team}/{year}/schedule")]
async fn get_schedule(path: web::Path<(String, String)>) -> Result<impl Responder> {
    // parse url params
    let (team, year) = path.into_inner();

    // Grab JSON data
    let data_result = file_actions::get_json();

    // Parse out data/error
    let data = match data_result {
        Ok(data) => data,
        Err(error) => {
            println!("ERROR: {:?}", error);
            return Ok(HttpResponse::InternalServerError().body("API Error"))},
    };

    // access relevant data
    let res = &data[&team][&year]["Schedule"];

    if res.is_null() {
        println!("{}", format!("Schedule Data was null for {} - {}", year, team));
        let res_msg = format!("Schedule for the {} - {} was not found.", year, team);
        return Ok(HttpResponse::NotFound().body(res_msg));
    }

    Ok(HttpResponse::Ok().body(res.to_string()))
}

