use std::collections::HashMap;
use crate::gateway::cli::presenter::NormalizedError;

#[derive(Debug)]
pub struct Flags {
    map: HashMap<String, Option<String>>,
}

const FLAGS_WITH_REQUIRED_ARG: [&str; 4] = [
    "from",
    "to",
    "name",
    "outter",
];

impl Flags {
    pub fn new() -> Flags {
        Flags {
            map: HashMap::new(),
        }
    }

    pub fn try_insert<F>(&mut self, flag: &str, get_arg: F) -> Result<(), NormalizedError>
    where
        F: FnOnce() -> Option<String>,
    {
        let sanitized_flag = &flag.replace('-', "")[..];
        let long_flag = if sanitized_flag.len() == 1 {
            match sanitized_flag {
                "n" => "name",
                _ => {
                    println!("{flag} is not a valid short flag.");
                    return Err(NormalizedError::InvalidFlag(None));
                }
            }
        } else {
            sanitized_flag
        };

        let is_arg_required = FLAGS_WITH_REQUIRED_ARG.contains(&long_flag);

        let arg = if is_arg_required {
            let arg = get_arg();
            if arg == None {
                return Err(NormalizedError::RequiredFlagArg(None));
            }
            arg
        } else {
            None
        };

        self.map.insert(long_flag.to_string(), arg);

        Ok(())
    }

    pub fn get(&self, flag: &str) -> &Option<String> {
        let result = self.map.get(flag);

        if let Some(flag) = result {
            return flag;
        } else {
            return &None;
        }
    }
}
