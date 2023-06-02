use crate::gateway::presenter::NormalizedError;

#[derive(Debug)]
pub enum Action {
    Help,
    Swallow,
    Spit,
    Treasure(TreasureSubAction),
}

impl Action {
    pub fn try_new<F>(action_str: &str, get_subaction: F) -> Result<Self, NormalizedError>
    where
        F: FnOnce() -> Option<String>,
    {
        if action_str.is_empty() {
            return Err(NormalizedError::EmptyAction(None));
        }

        match &action_str.to_lowercase()[..] {
            "help" => Ok(Action::Help),
            "swallow" => Ok(Action::Swallow),
            "swl" => Ok(Action::Swallow),
            "spit" => Ok(Action::Spit),
            "spt" => Ok(Action::Spit),
            "treasure" => match get_subaction() {
                Some(s) => {
                    let subaction = TreasureSubAction::try_new(&s)?;
                    Ok(Action::Treasure(subaction))
                }
                None => Err(NormalizedError::EmptyAction(None)),
            },
            _ => Err(NormalizedError::UnknownAction(None)),
        }
    }
}

/* =========================================================
SUB ACTIONS
========================================================= */

#[derive(Debug)]
pub enum TreasureSubAction {
    List,
    Name,
}

impl TreasureSubAction {
    fn try_new(subaction_str: &str) -> Result<TreasureSubAction, NormalizedError> {
        match &subaction_str[..] {
            "list" => Ok(TreasureSubAction::List),
            "name" => Ok(TreasureSubAction::Name),
            _ => Err(NormalizedError::EmptyAction(None)),
        }
    }
}
