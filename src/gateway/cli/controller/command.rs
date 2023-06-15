use super::{Action, Config, Flags};
use crate::{
    app::{HelpDto, PathDto, SimpleTreasureDto, TreasureDto},
    gateway::cli::presenter::NormalizedError,
};

#[derive(Debug)]
pub struct Command {
    pub action: Action,
    pub action_args: Vec<String>,
    pub flags: Flags,
}

impl Command {
    pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Command, NormalizedError> {
        args.next(); // path

        let mut action: Option<Action> = None;
        let mut action_args: Vec<String> = Vec::new();
        let mut flags = Flags::new();

        while let Some(arg) = args.next() {
            match arg {
                long_flag if long_flag.starts_with("--") => {
                    flags.try_insert(&long_flag, || args.next())?;
                }
                short_flags if short_flags.starts_with("-") => {
                    for (_, short_flag) in short_flags.chars().enumerate() {
                        if short_flag == '-' {
                            continue;
                        }
                        flags.try_insert(&short_flag.to_string(), || args.next())?;
                    }
                }
                arg => {
                    if action.is_none() {
                        action = Some(Action::try_new(&arg, || args.next())?);
                    } else {
                        action_args.push(arg)
                    }
                }
            }
        }

        if let Some(action) = action {
            Ok(Command {
                action,
                action_args,
                flags,
            })
        } else {
            Err(NormalizedError::EmptyAction(None))
        }
    }

    pub fn to_help_dto<'a>(&'a self) -> Result<HelpDto, NormalizedError> {
        let queried_command = self.action_args.get(0);

        Ok(HelpDto::new(queried_command.map(|x| &**x)))
    }

    pub fn to_path_dto<'a>(&'a self, config: &'a Config) -> Result<PathDto, NormalizedError> {
        let treasure_name_or_path = self.action_args.get(0);

        if treasure_name_or_path == None {
            return Err(NormalizedError::RequiredActionArg(None));
        }

        Ok(PathDto::new(
            &config.chest,
            treasure_name_or_path.unwrap(),
            self.flags.get("from"),
            self.flags.get("to"),
            self.flags.get("name"),
        ))
    }

    pub fn to_treasure_dto<'a>(
        &'a self,
        config: &'a Config,
    ) -> Result<TreasureDto, NormalizedError> {
        let treasure_name = self.action_args.get(0);
        let treasure_path = self.action_args.get(1);

        if treasure_name == None || treasure_path == None {
            return Err(NormalizedError::RequiredActionArg(None));
        }

        Ok(TreasureDto::new(
            &config.chest,
            treasure_name.unwrap(),
            treasure_path.unwrap(),
            self.flags.get("outter"),
        ))
    }

    pub fn to_simple_treasure_dto<'a>(
        &'a self,
        config: &'a Config,
    ) -> Result<SimpleTreasureDto, NormalizedError> {
        let treasure_name = self.action_args.get(0);

        if treasure_name == None {
            return Err(NormalizedError::RequiredActionArg(None));
        }

        Ok(SimpleTreasureDto::new(
            &config.chest,
            treasure_name.unwrap(),
        ))
    }
}
