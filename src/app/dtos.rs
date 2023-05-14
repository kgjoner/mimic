use std::{error::Error, fmt::Display};

pub struct PathDto<'a> {
    pub chest: &'a str,
    pub treasure_name_or_path: &'a str,
    pub mods: PathMods<'a>,
}

pub struct PathMods<'a> {
    pub from: &'a Option<String>,
    pub to: &'a Option<String>,
}

pub struct TreasureDto<'a> {
    pub chest: &'a str,
    pub treasure_name: &'a str,
    pub treasure_path: &'a str,
    pub mods: TreasureMods<'a>,
}

pub struct TreasureMods<'a> {
    pub outter_target_path: &'a Option<String>,
}


pub trait Interactor<'a> {
    type Input;

    fn execute(&self, input: Self::Input) -> Result<Box<dyn Display>, Box<dyn Error>>;
}