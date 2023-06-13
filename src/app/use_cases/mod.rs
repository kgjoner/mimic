use std::{error::Error, fmt::Display};
pub trait Interactor<'a> {
  type Input;

  fn execute(&self, input: Self::Input) -> Result<Box<dyn Display>, Box<dyn Error>>;
}

mod help;
pub use help::HelpInteractor;

mod list_treasures;
pub use list_treasures::ListTreasuresInteractor;

mod memorize_treasure;
pub use memorize_treasure::MemorizeTreasureInteractor;

mod spit;
pub use spit::SpitInteractor;

mod swallow;
pub use swallow::SwallowInteractor;
