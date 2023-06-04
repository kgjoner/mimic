mod controller;
pub use controller::{Action, Command, Config, TreasureSubAction};

mod presenter;
pub use presenter::{NormalizedError, NormalizedSuccess};

use crate::{
    app::{
        help::HelpInteractor, list_treasures::ListTreasuresInteractor,
        name_treasure::NameTreasureInteractor, spit::SpitInteractor, swallow::SwallowInteractor,
        Interactor,
    },
    domain::{Repos, TreasureRecordsRepoInterface},
};

pub struct CliGateway {
    repos: Repos,
    config: Config,
}

impl CliGateway {
    pub fn raise(treasure_records_repo: impl TreasureRecordsRepoInterface + 'static) -> CliGateway {
        let config = Config::load();
        CliGateway {
            repos: Repos {
                treasure_records: Box::new(treasure_records_repo),
            },
            config,
        }
    }

    pub fn cross(
        &self,
        args: impl Iterator<Item = String>,
    ) -> Result<NormalizedSuccess, NormalizedError> {
        let command = Command::parse(args)?;

        match command.action {
            Action::Help => self.execute(HelpInteractor::new(), command.to_help_dto()?),
            Action::Swallow => self.execute(
                SwallowInteractor::new(&self.repos),
                command.to_path_dto(&self.config)?,
            ),
            Action::Spit => self.execute(
                SpitInteractor::new(&self.repos),
                command.to_path_dto(&self.config)?,
            ),
            Action::Treasure(TreasureSubAction::Name) => self.execute(
                NameTreasureInteractor::new(&self.repos),
                command.to_treasure_dto(&self.config)?,
            ),
            Action::Treasure(TreasureSubAction::List) => self.execute(
                ListTreasuresInteractor::new(&self.repos),
                &self.config.chest[..],
            ),
        }
    }

    fn execute<'a, T>(
        &self,
        interactor: impl Interactor<'a, Input = T>,
        input: T,
    ) -> Result<NormalizedSuccess, NormalizedError> {
        let result = interactor.execute(input);

        match result {
            Err(err) => Err(NormalizedError::new(err)),
            Ok(res) => Ok(NormalizedSuccess::new(res)),
        }
    }
}
