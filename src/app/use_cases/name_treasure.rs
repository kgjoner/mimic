use crate::{
    app::{Interactor, TreasureDto, TreasureMods},
    domain::{TreasureRecordsRepoInterface, Repos},
};
use std::{error::Error, fmt::Display};

pub struct NameTreasureInteractor<'a> {
    treasure_records_repo: &'a Box<dyn TreasureRecordsRepoInterface>,
}

impl<'a> NameTreasureInteractor<'a> {
    pub fn new(repos: &'a Repos) -> NameTreasureInteractor {
        NameTreasureInteractor {
            treasure_records_repo: &repos.treasure_records,
        }
    }
}

impl<'a> Interactor<'a> for NameTreasureInteractor<'a> {
    type Input = TreasureDto<'a>;

    fn execute(
        &self,
        TreasureDto {
            chest,
            treasure_name,
            treasure_path,
            mods: TreasureMods { outter_target_path },
        }: TreasureDto,
    ) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let compartment_path = treasure_path.to_string();
        let outter_target_path = if let Some(p) = outter_target_path {
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

        let result = format!("<talk>Alright, that's a good name. I know {} now!<r>", treasure_name);
        Ok(Box::new(result))
    }
}
