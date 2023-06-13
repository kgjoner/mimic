use crate::{
    app::{Interactor, TreasureDto},
    domain::{Repos, TreasureRecordsRepoInterface},
};
use std::{error::Error, fmt::Display};

pub struct MemorizeTreasureInteractor<'a> {
    treasure_records_repo: &'a Box<dyn TreasureRecordsRepoInterface>,
}

impl<'a> MemorizeTreasureInteractor<'a> {
    pub fn new(repos: &'a Repos) -> MemorizeTreasureInteractor {
        MemorizeTreasureInteractor {
            treasure_records_repo: &repos.treasure_records,
        }
    }
}

impl<'a> Interactor<'a> for MemorizeTreasureInteractor<'a> {
    type Input = TreasureDto<'a>;

    fn execute(&self, input: TreasureDto) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let TreasureDto {
            chest,
            treasure_name,
            treasure_path,
            mods,
        } = input;

        let compartment_path = treasure_path.to_string();
        let outter_target_path = if let Some(p) = mods.outter_target_path {
            p.to_string()
        } else {
            treasure_path.to_string()
        };

        self.treasure_records_repo.upsert_record(
            chest,
            treasure_name.to_string(),
            compartment_path,
            Some(outter_target_path),
        )?;

        let result = format!(
            "<talk>Alright, that's a good name. I know {} now!<r>",
            treasure_name
        );
        Ok(Box::new(result))
    }
}
