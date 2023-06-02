use crate::constants::BASE_PATH;
use std::{collections::HashMap, env, error::Error, fmt::Display, path::Path};

#[derive(Debug)]
pub struct TreasureRecords {
    pub chest: String,
    paths_map: HashMap<String, TreasurePaths>,
}

impl TreasureRecords {
    pub fn from_str(chest: &str, content: &str) -> TreasureRecords {
        let mut treasures = HashMap::new();

        for line in content.lines() {
            if let [treasure_name, treasure_dir] =
                &line.split("=").map(|str| str.trim()).collect::<Vec<&str>>()[..]
            {
                treasures.insert(treasure_name.to_string(), TreasurePaths::new(treasure_dir));
            }
        }

        TreasureRecords {
            chest: chest.to_string(),
            paths_map: treasures,
        }
    }

    pub fn get_paths(&self, treasure_name: &str) -> Option<&TreasurePaths> {
        self.paths_map.get(treasure_name)
    }

    pub fn upsert(
        &mut self,
        treasure_name: String,
        compartment_path: String,
        outter_path: Option<String>,
    ) {
        let outter_target_path = outter_path.unwrap_or(compartment_path.clone());
        self.paths_map.insert(
            treasure_name,
            TreasurePaths {
                compartment_path,
                outter_target_path,
            },
        );
    }

    pub fn into_string(&self) -> String {
        let mut content = String::new();
        for (treasure_name, paths) in self.paths_map.iter() {
            let line = format!(
                "{treasure_name}={},{}",
                paths.compartment_path, paths.outter_target_path
            ) + "\n";
            content.push_str(&line);
        }
        content
    }

    pub fn is_listed(&self, treasure_name: &str) -> bool {
        self.paths_map.contains_key(treasure_name)
    }

    pub fn is_stored(&self, treasure_name: &str) -> bool {
        let treasure_paths = &self.get_paths(treasure_name);

        if let Some(treasure_paths) = treasure_paths {
            Path::new(&treasure_paths.compartment_full_path(&self.chest)).exists()
        } else {
            false
        }
    }
}

impl Display for TreasureRecords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_string())
    }
}

#[derive(Debug)]
pub struct TreasurePaths {
    pub compartment_path: String,
    pub outter_target_path: String,
}

impl TreasurePaths {
    fn new(dir: &str) -> TreasurePaths {
        if let [compartment_path, outter_target_path] =
            &dir.split(",").map(|str| str.trim()).collect::<Vec<&str>>()[..]
        {
            TreasurePaths {
                compartment_path: compartment_path.to_string(),
                outter_target_path: outter_target_path.to_string(),
            }
        } else {
            TreasurePaths {
                compartment_path: String::from(dir),
                outter_target_path: String::from(dir),
            }
        }
    }

    pub fn compartment_full_path(&self, chest: &str) -> String {
        format!(
            "{}/{BASE_PATH}/chests/{chest}/{}",
            env::var("HOME").unwrap(),
            self.compartment_path
        )
    }
}

impl Clone for TreasurePaths {
    fn clone(&self) -> TreasurePaths {
        TreasurePaths {
            compartment_path: self.compartment_path.clone(),
            outter_target_path: self.outter_target_path.clone(),
        }
    }
}

pub trait TreasureRecordsRepoInterface {
    fn get_records(&self, chest: &str) -> Result<TreasureRecords, Box<dyn Error>>;
    fn upsert_record(
        &self,
        chest: &str,
        treasure_name: String,
        compartment_path: String,
        outter_path: Option<String>,
    ) -> Result<(), Box<dyn Error>>;
}
