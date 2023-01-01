use std::env;

/// Determine if runtime environment is dev|prod|etc.
pub fn is_dev() -> bool {
    let env = env::var("RUN_ENV").expect("Runtime env not set");
    env == "dev"
}
