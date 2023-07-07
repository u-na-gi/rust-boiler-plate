
use actix_web::{get, HttpResponse, Responder};
use crate::scenario::health_check_scenario;


#[get("/health-check")]
pub async fn health_check() -> impl Responder {
    let res = health_check_scenario::invoke();
    let json = serde_json::to_string(&res).unwrap();
    HttpResponse::Ok().body(json)
}
