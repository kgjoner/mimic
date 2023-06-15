use crate::{
    app::{Interactor, SimpleTreasureDto},
    domain::{Repos, TreasureRecordsRepoInterface},
};
use std::{error::Error, fmt::Display};

pub struct ForgetTreasureInteractor<'a> {
    treasure_records_repo: &'a Box<dyn TreasureRecordsRepoInterface>,
}

impl<'a> ForgetTreasureInteractor<'a> {
    pub fn new(repos: &'a Repos) -> ForgetTreasureInteractor {
        ForgetTreasureInteractor {
            treasure_records_repo: &repos.treasure_records,
        }
    }
}

impl<'a> Interactor<'a> for ForgetTreasureInteractor<'a> {
    type Input = SimpleTreasureDto<'a>;

    fn execute(&self, input: SimpleTreasureDto) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let SimpleTreasureDto {
            chest,
            treasure_name,
        } = input;

        self.treasure_records_repo
            .remove_record(chest, treasure_name.to_string())?;

        let result =
            format!("<talk>Uhm... I used to know it, but I dunno it anymore!<r>");
        Ok(Box::new(result))
    }
}
