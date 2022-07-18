pub mod schema;
pub mod models;

use std::{env, time::SystemTime};
use std::io::Read;

use actix_web::{middleware::Logger, HttpServer, web, App, post, HttpResponse, error};
use diesel::{r2d2::{ConnectionManager, self}, PgConnection, prelude::*};
use env_logger::Env;
use quick_xml::{Reader, events::Event, de::{from_str, DeError}};
use serde_json::{Value, json};
use models::*;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;


#[post("/data")]
async fn send(pool: web::Data<DbPool>, body: String) -> actix_web::Result<HttpResponse> {
    let arrived_at = SystemTime::now();

    let required_data: RequiredData = from_str(&body)
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;
    let data = parse_request(&body);
    let json_data = from_str::<JsonData>(&body);
    
    let mut validation_error = None;
    let mut validation_ok = true;

    let keys_str = match json_data {
        Ok(ref str) => match serde_json::from_str::<Value>(&str.str_field6) {
            Ok(value) => if !value.is_array() {
                Some(get_keys_string_from_value(&value))  
            } else {
                None
            },
            Err(_) => None,
        },
        Err(ref e) => {
            validation_error = Some(e.to_string());
            None
        },
    };

    match data {
        Err(e) => {
            validation_error = Some(e.to_string());
            validation_ok = false;
        }
        _ => (),
    }

    let data = NewTrackedData {
        arrived_at,
        event_type: required_data.event_type,
        found_fields: keys_str,
        raw_json: json_data.map(|v| v.str_field6).ok(),
        raw_xml: body,
        source_machine: required_data.str_field2,
        validation_ok,
        validation_error: validation_error.clone(),
    };

    let _add_entry = web::block(move || {
        use schema::tracked_data::dsl::*;
        
        let mut conn = pool.get().unwrap(); // unwrap() should be replaced!
    
        diesel::insert_into(tracked_data)
            .values(&data)
            .execute(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(json!({
        "validation_ok": validation_ok,
        "validation_error": validation_error.unwrap_or("".to_owned())
    }).to_string()))
}

fn parse_request(request_str: &str) -> Result<XmlData, DeError> {
    let request: XmlData = from_str(request_str)?;

    let mut reader = Reader::from_str(request_str);
    reader.trim_text(true);

    loop {
        match reader.read_event_unbuffered() {
            Ok(Event::Start(ref e)) => {
                let mut s = String::new();
                e.name().read_to_string(&mut s).unwrap();
                
                log::debug!("{}", s);

                if s != "request" {
                    return Err(DeError::Custom("missing root field `request`".into()));
                } else {
                    break;
                }
            }
            Err(e) => return Err(DeError::Xml(e)),
            _ => (),
        }
    }
 
    Ok(request)
}

fn get_keys_string_from_value(value: &Value) -> String {
    // maybe collect() should be used
    value.as_object().unwrap()
        .keys()
        .fold(String::new(), |acc, x| {
            format!("{}, {}", acc, x)
        })
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    env_logger::init_from_env(Env::default().default_filter_or("trace")); //info

    let db_url = env::var("DATABASE_URL").unwrap();
    let host = env::var("SERVER_HOST").unwrap();
    let port: u16 = env::var("SERVER_PORT").unwrap().parse().unwrap();

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .service(send))
        .bind((host, port))?
        .run()
        .await
}