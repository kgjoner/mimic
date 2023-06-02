use crate::app::{HelpDto, Interactor};
use std::{error::Error, fmt::Display, fs};

pub struct HelpInteractor;

impl HelpInteractor {
    pub fn new() -> HelpInteractor {
        HelpInteractor
    }
}

impl<'a> Interactor<'a> for HelpInteractor {
    type Input = HelpDto<'a>;

    fn execute(
        &self,
        HelpDto { queried_command }: HelpDto,
    ) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let doc = fs::read_to_string("doc.txt")?;

        let mut result = String::new();

        if let Some(cmd) = queried_command {
            let mut doc_lines = doc.lines().peekable();
            let mut is_capturing = false;

            while let Some(line) = doc_lines.next() {
                if line.contains("====") {
                    if is_capturing {
                        break;
                    };

                    let next_line = doc_lines.peek();
                    if next_line == None {
                        break;
                    }
                    if next_line.unwrap().contains(cmd) {
                        is_capturing = true;
                    }
                } else if is_capturing {
                    result += &*format!("\n{line}");
                }
            }

            if result.is_empty() {
              result = format!("{:?} is an invalid command", cmd);
            }
        } else {
            result = doc.to_string();
        }

        Ok(Box::new(result))
    }
}
