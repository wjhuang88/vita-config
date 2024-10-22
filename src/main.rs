mod config;
mod datasource;
mod errors;

use ntex::web;
use std::{fs::File, path::PathBuf};

use config::DataSourceSpec;
use datasource::{DataSource, Value};

#[web::get("/")]
async fn hello() -> impl web::Responder {
    let conf_path = PathBuf::from("tests/config/test_datasource.yaml");
    let conf_file = File::open(conf_path).unwrap();
    let conf_instance = config::load(conf_file).unwrap();

    println!("config: {:?}", conf_instance);

    let db_spec: DataSourceSpec = conf_instance.map_spec().unwrap();
    let db = datasource::connect(&db_spec).unwrap();

    let value: Vec<String> = vec![];
    web::HttpResponse::Ok().json(&value)
}

#[ntex::main]
async fn main() -> errors::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .wrap(web::middleware::Compress::default())
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}
