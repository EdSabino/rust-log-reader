extern crate config;

use config::Config;

pub fn config() -> Config {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Setting")).unwrap();
    settings
}