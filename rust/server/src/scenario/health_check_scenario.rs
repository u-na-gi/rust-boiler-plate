use serde::{Serialize};

#[derive(Serialize)]
pub struct HealthCheckResponse {
    message: String,
}

pub fn invoke() -> HealthCheckResponse {
    HealthCheckResponse {
        message: String::from("I am alive!")
    }
}



