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

/// Get app name configured
pub fn get_app_name() -> String {
    env::var("APP_NAME").unwrap_or(String::from("codefee-works-api"))
}
