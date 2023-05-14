mod error;
pub use error::*;

mod success;
pub use success::*;

pub fn print_message(message: &str, is_err: bool) {
  let formatted_message = message
      .replace("<talk>", "\x1b[3;92m")
      .replace("<u>", "\x1b[4m")
      .replace("<r>", "\x1b[0m");

  if is_err {
      eprintln!("{formatted_message}");
  } else {
      println!("{formatted_message}");
  }
}
