use crate::constants::DEFAULT_CHEST;

pub struct Config {
    pub chest: String,
}
impl Config {
    pub fn load() -> Config {
        Config {
            chest: DEFAULT_CHEST.to_string(),
        }
    }
}
