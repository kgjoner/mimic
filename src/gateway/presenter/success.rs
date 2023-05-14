use std::fmt::Display;

use super::print_message;

pub struct NormalizedSuccess(String);

impl NormalizedSuccess {
  pub fn new(res: impl Display) -> NormalizedSuccess {
    NormalizedSuccess(res.to_string())
  }

  pub fn print(&self) {
      let message = &format!("{:?}", self.0);
      print_message(message, false);
  }
}