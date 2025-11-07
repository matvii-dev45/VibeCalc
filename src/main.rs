use vibe_calc::app;
use vibe_calc::utils::config;

fn main() {
    let config = config::read_config().unwrap_or_else(|e| {
        eprintln!("Failed to read config: {}, using defaults", e);
        config::create_default_config()
    });

    if let Err(e) = app::run(config) {
        eprintln!("Application error: {}", e);
    }
}