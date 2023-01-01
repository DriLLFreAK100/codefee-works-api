use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use r2d2::{Error, Pool, PooledConnection};
use std::env;

use super::error::AppError;

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_pool() -> PostgresPool {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Error building connection pool")
}

pub trait PoolConnection {
    /// Run operation (closure) by passing in the mutable ref of an actual connection
    fn run<F, D>(&mut self, func: F) -> Result<D, AppError>
    where
        F: Fn(&mut PgConnection) -> Result<D, AppError>;
}

impl PoolConnection for Result<PooledConnection<ConnectionManager<PgConnection>>, Error> {
    fn run<F, D>(&mut self, func: F) -> Result<D, AppError>
    where
        F: Fn(&mut PgConnection) -> Result<D, AppError>,
    {
        match self.as_deref_mut() {
            Ok(con) => func(con),
            _ => Err(AppError::ServerError),
        }
    }
}
