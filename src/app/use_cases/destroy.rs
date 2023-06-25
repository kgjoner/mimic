use crate::{
    app::{Interactor, SimpleTreasureDto},
    domain::{Repos, TreasurePaths, TreasureRecordsRepoInterface},
};
use std::{error::Error, fmt::Display, fs, path::Path};

pub struct DestroyInteractor<'a> {
    treasure_records_repo: &'a Box<dyn TreasureRecordsRepoInterface>,
}

impl<'a> DestroyInteractor<'a> {
    pub fn new(repos: &'a Repos) -> DestroyInteractor {
        DestroyInteractor {
            treasure_records_repo: &repos.treasure_records,
        }
    }
}

impl<'a> Interactor<'a> for DestroyInteractor<'a> {
    type Input = SimpleTreasureDto<'a>;

    fn execute(&self, input: SimpleTreasureDto) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let SimpleTreasureDto {
            chest,
            treasure_name: treasure_name_or_path,
        } = input;

        let records = self.treasure_records_repo.get_records(chest)?;

        let compartment_full_path = if records.is_listed(treasure_name_or_path) {
            let treasure_name = treasure_name_or_path;
            records
                .get_paths(treasure_name)
                .unwrap()
                .compartment_full_path(chest)
        } else {
            let treasure_path = treasure_name_or_path.to_string();
            TreasurePaths {
                compartment_path: treasure_path.clone(),
                outter_target_path: treasure_path,
            }
            .compartment_full_path(chest)
        };

        let compartment_full_path = Path::new(&compartment_full_path);

        if compartment_full_path.is_file() {
            fs::remove_file(compartment_full_path)?;
        } else {
            fs::remove_dir_all(compartment_full_path)?;
        }

        let result = format!("<talk>Burrrp! Ow, such a relief!<r>");
        Ok(Box::new(result))
    }
}
