use crate::gateway::cli::presenter::NormalizedError;

#[derive(Debug)]
pub enum Action {
    Destroy,
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
            return Err(NormalizedError::EmptyAction(get_error_tip_msg(None, None)));
        }

        match &action_str.to_lowercase()[..] {
            "destroy" => Ok(Action::Destroy),
            "help" => Ok(Action::Help),
            "swallow" | "swl" => Ok(Action::Swallow),
            "spit" | "spt" => Ok(Action::Spit),
            "treasure" => match get_subaction() {
                Some(s) => {
                    let subaction = TreasureSubAction::try_new(&s)?;
                    Ok(Action::Treasure(subaction))
                }
                None => Err(NormalizedError::EmptyAction(get_error_tip_msg(
                    None,
                    Some("treasure"),
                ))),
            },
            unknown_action => Err(NormalizedError::UnknownAction(get_error_tip_msg(
                Some(unknown_action),
                None,
            ))),
        }
    }
}

/* =========================================================
SUB ACTIONS
========================================================= */

#[derive(Debug)]
pub enum TreasureSubAction {
    List,
    Memorize,
    Forget,
}

impl TreasureSubAction {
    fn try_new(subaction_str: &str) -> Result<TreasureSubAction, NormalizedError> {
        match &subaction_str[..] {
            "list" => Ok(TreasureSubAction::List),
            "memorize" | "memo" => Ok(TreasureSubAction::Memorize),
            "forget" => Ok(TreasureSubAction::Forget),
            unknown_subaction => Err(NormalizedError::UnknownAction(get_error_tip_msg(
                Some(unknown_subaction),
                Some("treasure"),
            ))),
        }
    }
}

/* =========================================================
UTILS
========================================================= */

fn get_error_tip_msg(unknown_input: Option<&str>, parent_action: Option<&str>) -> Option<String> {
    let mut msg = "Call <u>help<r> to check available commands".to_string();

    if let Some(action) = parent_action {
        msg = format!("{} for <u>{}<r> action.", msg, action);
    } else {
        msg += ".";
    }

    if let Some(input) = unknown_input {
        msg = format!("<u>{input}<r> is not a valid command. {}", msg);
    }

    Some(msg)
}
