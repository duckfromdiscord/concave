use actix_web::{App, HttpServer, middleware, web};
use clap::{Arg, Command};
use concave::db::AppState;

use deadpool_diesel::sqlite::{Manager, Pool};
use reqwest::Client;

const DB_URL: &str = "sqlite://db.sqlite";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let manager = Manager::new(DB_URL, deadpool_diesel::Runtime::Tokio1);

    let pool = Pool::builder(manager)
        .max_size(8)
        .build()
        .expect("DB pool build error");

    let matches = Command::new("concave")
        .arg(
            Arg::new("port")
                .short('p')
                .value_name("PORT")
                .help("Listen port"),
        )
        .arg(
            Arg::new("ip")
                .short('j')
                .value_name("IP")
                .help("Listen address"),
        )
        .get_matches();

    let listen_ip: String = matches
        .get_one::<String>("ip")
        .unwrap_or(&"127.0.0.1".to_string())
        .to_string();

    let listen_port: u16 = matches
        .get_one::<String>("port")
        .unwrap_or(&"8009".to_string())
        .parse::<u16>()
        .expect("Invalid port");

    log::info!(
        "starting HTTP server at http://{}:{}",
        listen_ip,
        listen_port
    );

    let state = web::Data::new(AppState {
        db: pool.clone(),
        client: Client::new(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(concave::app::add_artist)
            .service(concave::app::search_artist)
            .service(concave::app::list_artists)
            .service(concave::app::try_image)
            .service(concave::app::delete_artist)
            .service(concave::app::list_apis)
            .service(concave::app::add_api)
            .service(concave::app::delete_api)
            .service(concave::app::get_sources)
            .service(concave::app::lookup_at)
            .service(actix_files::Files::new("/", "./frontend/dist").index_file("index.html"))
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind((listen_ip, listen_port))?
    .run()
    .await
}
