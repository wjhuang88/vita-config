mod config;
mod datasource;
mod errors;

use std::{fs::File, path::PathBuf};

use config::DataSourceSpec;
use datasource::{DataSource, Value};

fn main() {
    let conf_path = PathBuf::from("tests/config/test_datasource.yaml");
    let conf_file = File::open(conf_path).unwrap();
    let conf_instance = config::load(conf_file).unwrap();

    println!("config: {:?}", conf_instance);

    let db_spec: DataSourceSpec = conf_instance.map_spec().unwrap();
    let db = datasource::connect(&db_spec).unwrap();

    if let Some(query) = db_spec.query {
        // let name = query[1].name.clone();
        let sql = query[1].sql.clone();
        let result = if let Some(params) = &query[1].params {
            let c_params: Vec<Value> = params
                .iter()
                .map(|p| match p.p_type {
                    config::ParamType::Float => Value::Null,
                    config::ParamType::Integer => Value::Integer(3000),
                    config::ParamType::String => Value::Null,
                })
                .collect();
            db.query(sql, c_params.as_slice()).unwrap()
        } else {
            db.raw_query(sql).unwrap()
        };

        println!("result: {:?}", result);
    }
}
