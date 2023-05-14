mod treasure_records;

pub use treasure_records::*;

pub struct Repos {
  pub treasure_records: Box<dyn TreasureRecordsRepoInterface>,
}
