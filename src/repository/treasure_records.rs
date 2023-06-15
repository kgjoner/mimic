use crate::constants::{BASE_PATH, TREASURES_FILE};
use crate::domain::{TreasureRecords, TreasureRecordsRepoInterface};
use std::env;
use std::io::Write;
use std::{error::Error, fs, path::Path};

pub struct TreasureRecordsRepo(());

impl TreasureRecordsRepo {
    pub fn new() -> TreasureRecordsRepo {
        TreasureRecordsRepo(())
    }

    fn get_treasure_file_path(&self) -> String {
        format!("{}/{BASE_PATH}/{TREASURES_FILE}", env::var("HOME").unwrap())
    }

    fn write_on_treasure_file(&self, records: TreasureRecords) -> Result<(), Box<dyn Error>> {
        let content = records.into_string();

        let treasure_file_path = self.get_treasure_file_path();
        let path = Path::new(&treasure_file_path);
        let path_exists = path.try_exists().unwrap_or(false);
        let mut file = if path_exists {
            fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(treasure_file_path)?
        } else {
            fs::create_dir_all(path.parent().unwrap())?;
            fs::File::create(treasure_file_path)?
        };

        file.write_all(content.as_bytes())?;

        Ok(())
    }
}

impl TreasureRecordsRepoInterface for TreasureRecordsRepo {
    fn get_records(&self, chest: &str) -> Result<TreasureRecords, Box<dyn Error>> {
        let treasure_file_path = self.get_treasure_file_path();
        let path_exists = Path::new(&treasure_file_path).try_exists().unwrap_or(false);
        let content = if path_exists {
            fs::read_to_string(treasure_file_path)?
        } else {
            String::new()
        };

        let records = TreasureRecords::from_str(chest, &content);

        Ok(records)
    }

    fn upsert_record(
        &self,
        chest: &str,
        treasure_name: String,
        compartment_path: String,
        outter_target_path: Option<String>,
    ) -> Result<(), Box<dyn Error>> {
        let mut records = self.get_records(chest)?;
        records.upsert(treasure_name, compartment_path, outter_target_path);

        self.write_on_treasure_file(records)
    }

    fn remove_record(&self, chest: &str, treasure_name: String) -> Result<(), Box<dyn Error>> {
        let mut records = self.get_records(chest)?;
        records.remove(treasure_name);

        self.write_on_treasure_file(records)
    }
}
