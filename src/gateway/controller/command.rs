use super::{Action, Config, Flags};
use crate::{
    app::{HelpDto, PathDto, PathMods, TreasureDto, TreasureMods},
    gateway::presenter::NormalizedError,
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

        Ok(HelpDto {
            queried_command: queried_command.map(|x| &**x),
        })
    }

    pub fn to_path_dto<'a>(&'a self, config: &'a Config) -> Result<PathDto, NormalizedError> {
        let treasure_name_or_path = self.action_args.get(0);

        if treasure_name_or_path == None {
            return Err(NormalizedError::RequiredActionArg(None));
        }

        Ok(PathDto {
            chest: &config.chest,
            treasure_name_or_path: treasure_name_or_path.unwrap(),
            mods: PathMods {
                from: self.flags.get("from"),
                to: self.flags.get("to"),
            },
        })
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

        Ok(TreasureDto {
            chest: &config.chest,
            treasure_name: treasure_name.unwrap(),
            treasure_path: treasure_path.unwrap(),
            mods: TreasureMods {
                outter_target_path: self.flags.get("outter"),
            },
        })
    }
}
