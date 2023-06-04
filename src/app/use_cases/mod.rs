use std::{error::Error, fmt::Display};
pub trait Interactor<'a> {
  type Input;

  fn execute(&self, input: Self::Input) -> Result<Box<dyn Display>, Box<dyn Error>>;
}

mod help;
pub use help::HelpInteractor;

mod list_treasures;
pub use list_treasures::ListTreasuresInteractor;

mod name_treasure;
pub use name_treasure::NameTreasureInteractor;

mod spit;
pub use spit::SpitInteractor;

mod swallow;
pub use swallow::SwallowInteractor;
