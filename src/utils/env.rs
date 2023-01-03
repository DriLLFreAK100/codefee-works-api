use dotenv::dotenv;
use std::env;

/// Init env variables
pub fn init_env() {
    dotenv().ok().expect("Env init error");
}

/// Determine if runtime environment is dev|prod|etc.
pub fn is_dev() -> bool {
    let env = env::var("RUN_ENV").expect("Runtime env not set");
    env == "dev"
}

/// Get host port tuple for starting Actix server
pub fn get_host_port() -> (String, u16) {
    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT")
        .expect("Port not set")
        .parse::<u16>()
        .unwrap();

    (host, port)
}

/// Get log level - `info`, `error`, etc.
pub fn get_log_level() -> String {
    env::var("LOG_LEVEL").expect("Log level not set")
}
