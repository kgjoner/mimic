use crate::{
    app::{PathDto, PathMods, Interactor},
    domain::{Repos, TreasurePaths, TreasureRecordsRepoInterface},
};
use std::{error::Error, fs, fmt::Display, path::Path};

pub struct SpitInteractor<'a> {
    treasure_records_repo: &'a Box<dyn TreasureRecordsRepoInterface>,
}

impl<'a> SpitInteractor<'a> {
    pub fn new(repos: &'a Repos) -> SpitInteractor {
        SpitInteractor {
            treasure_records_repo: &repos.treasure_records,
        }
    }
}

impl<'a> Interactor<'a> for SpitInteractor<'a> {
    type Input = PathDto<'a>;

    fn execute(
        &self,
        PathDto {
            chest,
            treasure_name_or_path,
            mods: PathMods { from, to, name:_ },
        }: PathDto,
    ) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let records = self.treasure_records_repo.get_records(chest)?;

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
            paths.compartment_path = from.to_string();
        }
        if let Some(to) = to {
            paths.outter_target_path = to.to_string();
        }

        let outter_target_path = Path::new(&paths.outter_target_path);
        if !outter_target_path.try_exists().unwrap_or(false) {
            fs::create_dir_all(outter_target_path.parent().unwrap())?;
        }

        fs::copy(
            &paths.compartment_full_path(chest),
            outter_target_path,
        )?;

        Ok(Box::new("<talk>Yuck! There we go... Can I bite something now?<r>"))
    }
}
