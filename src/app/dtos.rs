pub struct HelpDto<'a> {
    pub queried_command: Option<&'a str>,
}

impl<'a> HelpDto<'a> {
    pub fn new(queried_command: Option<&'a str>) -> HelpDto<'a> {
        HelpDto { queried_command }
    }
}

pub struct PathDto<'a> {
    pub chest: &'a str,
    pub treasure_name_or_path: &'a str,
    pub mods: PathMods<'a>,
}

impl<'a> PathDto<'a> {
    pub fn new(
        chest: &'a str,
        treasure_name_or_path: &'a str,
        from: &'a Option<String>,
        to: &'a Option<String>,
        name: &'a Option<String>,
    ) -> PathDto<'a> {
        PathDto {
            chest,
            treasure_name_or_path,
            mods: PathMods { from, to, name },
        }
    }
}

pub struct PathMods<'a> {
    pub from: &'a Option<String>,
    pub to: &'a Option<String>,
    pub name: &'a Option<String>,
}

pub struct TreasureDto<'a> {
    pub chest: &'a str,
    pub treasure_name: &'a str,
    pub treasure_path: &'a str,
    pub mods: TreasureMods<'a>,
}

impl<'a> TreasureDto<'a> {
    pub fn new(
        chest: &'a str,
        treasure_name: &'a str,
        treasure_path: &'a str,
        outter_target_path: &'a Option<String>,
    ) -> TreasureDto<'a> {
        TreasureDto {
            chest,
            treasure_name,
            treasure_path,
            mods: TreasureMods { outter_target_path },
        }
    }
}

pub struct TreasureMods<'a> {
    pub outter_target_path: &'a Option<String>,
}

pub struct SimpleTreasureDto<'a> {
    pub chest: &'a str,
    pub treasure_name: &'a str,
}

impl<'a> SimpleTreasureDto<'a> {
    pub fn new(chest: &'a str, treasure_name: &'a str) -> SimpleTreasureDto<'a> {
        SimpleTreasureDto {
            chest,
            treasure_name,
        }
    }
}
