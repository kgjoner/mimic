use crate::{domain::{TreasureRecordsRepoInterface, Repos}, app::Interactor};
use std::{error::Error, fmt::Display};

pub struct ListTreasuresInteractor<'a> {
    treasure_records_repo: &'a Box<dyn TreasureRecordsRepoInterface>,
}

impl<'a> ListTreasuresInteractor<'a> {
    pub fn new(repos: &'a Repos) -> ListTreasuresInteractor {
        ListTreasuresInteractor {
            treasure_records_repo: &repos.treasure_records,
        }
    }
}

impl<'a> Interactor<'a> for ListTreasuresInteractor<'a> {
    type Input = &'a str;

    fn execute(&self, chest: &str) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let result = self.treasure_records_repo.get_records(chest)?;
        Ok(Box::new(result))
    }
}
