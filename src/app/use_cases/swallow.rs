use crate::{
    app::{PathDto, PathMods, Interactor},
    domain::{Repos, TreasurePaths, TreasureRecordsRepoInterface},
};
use std::{error::Error, fs, fmt::Display, path::Path};

pub struct SwallowInteractor<'a> {
    treasure_records_repo: &'a Box<dyn TreasureRecordsRepoInterface>,
}

impl<'a> SwallowInteractor<'a> {
    pub fn new(repos: &'a Repos) -> SwallowInteractor {
        SwallowInteractor {
            treasure_records_repo: &repos.treasure_records,
        }
    }
}

impl<'a> Interactor<'a> for SwallowInteractor<'a> {
    type Input = PathDto<'a>;

    fn execute(
        &self,
        PathDto {
            chest,
            treasure_name_or_path,
            mods: PathMods { from, to },
        }: PathDto,
    ) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let records = self.treasure_records_repo.get_records(chest).unwrap();

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

        if let Some(from) = from {
            paths.outter_target_path = from.to_string();
        }
        if let Some(to) = to {
            paths.compartment_path = to.to_string();
        }

        let compartment_full_path = paths.compartment_full_path(chest);
        let compartment_full_path = Path::new(&compartment_full_path);
        if !compartment_full_path.try_exists().unwrap_or(false) {
            fs::create_dir_all(compartment_full_path.parent().unwrap())?;
        }

        fs::copy(
            &paths.outter_target_path,
            compartment_full_path,
        )?;

        Ok(Box::new(""))
    }
}
