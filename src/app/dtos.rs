pub struct HelpDto<'a> {
    pub queried_command: Option<&'a str>
}

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
