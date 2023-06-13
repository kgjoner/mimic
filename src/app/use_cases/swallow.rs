use crate::{
    app::{Interactor, PathDto, TreasureDto, TreasureMods},
    domain::{Repos, TreasurePaths},
};
use std::{error::Error, fmt::Display, fs, path::Path};

use super::NameTreasureInteractor;

pub struct SwallowInteractor<'a> {
    repos: &'a Repos,
}

impl<'a> SwallowInteractor<'a> {
    pub fn new(repos: &'a Repos) -> SwallowInteractor {
        SwallowInteractor { repos }
    }
}

impl<'a> Interactor<'a> for SwallowInteractor<'a> {
    type Input = PathDto<'a>;

    fn execute(&self, input: PathDto) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let PathDto {
            chest,
            treasure_name_or_path,
            mods,
        } = input;

        let records = self.repos.treasure_records.get_records(chest).unwrap();

        let mut paths = if records.is_listed(treasure_name_or_path) {
            let treasure_name = treasure_name_or_path;
            records.get_paths(treasure_name).unwrap().clone()
        } else {
            let treasure_path = treasure_name_or_path.to_string();
            TreasurePaths {
                compartment_path: treasure_path.clone(),
                outter_target_path: treasure_path,
            }
        };

        if let Some(from) = mods.from {
            paths.outter_target_path = from.to_string();
        }
        if let Some(to) = mods.to {
            paths.compartment_path = to.to_string();
        }

        if let Some(name) = mods.name {
            let interactor = NameTreasureInteractor::new(&self.repos);
            interactor.execute(TreasureDto {
                chest,
                treasure_name: name,
                treasure_path: &paths.compartment_path,
                mods: TreasureMods {
                    outter_target_path: &Some(paths.outter_target_path.to_string()),
                },
            })?;
        }

        let compartment_full_path = paths.compartment_full_path(chest);
        let compartment_full_path = Path::new(&compartment_full_path);
        if !compartment_full_path.try_exists().unwrap_or(false) {
            fs::create_dir_all(compartment_full_path.parent().unwrap())?;
        }

        fs::copy(&paths.outter_target_path, compartment_full_path)?;

        Ok(Box::new("<talk>Yum... Delicious!<r>"))
    }
}
