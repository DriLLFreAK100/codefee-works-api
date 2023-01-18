use crate::utils::env::is_dev;
use actix_cors::Cors;
use std::env;

/// Configre CORS settings
pub fn with_cors() -> Cors {
    match is_dev() {
        true => Cors::permissive(),
        false => Cors::default()
            .allowed_origin(&*env::var("CORS_ALLOWED").expect("Cors allowed list not set")),
    }
}
