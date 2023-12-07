use actix_web::{App, HttpServer};

use nba_api_rust::endpoints::index;
use nba_api_rust::endpoints::get_team_year;
use nba_api_rust::endpoints::get_roster;
use nba_api_rust::endpoints::get_schedule;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
