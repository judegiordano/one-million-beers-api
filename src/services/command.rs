#[derive(Debug, PartialEq, Eq)]
pub enum SlashCommand {
    Ping,
    Help,
    BeerDown,
    BeerCount,
    BeerTab,
    // unhandled
    Unhandled,
}

impl SlashCommand {
    #[allow(clippy::inherent_to_string)]
    pub fn name(&self) -> String {
        match self {
            Self::Ping => "ping".to_string(),
            Self::Help => "help".to_string(),
            Self::BeerDown => "beer-down".to_string(),
            Self::BeerCount => "beer-count".to_string(),
            Self::BeerTab => "beer-tab".to_string(),
            Self::Unhandled => "unhandled".to_string(),
        }
    }

    pub fn description(&self) -> String {
        match self {
            Self::Ping => "Responds with 'pong'.".to_string(),
            Self::Help => "List the currently supported commands.".to_string(),
            Self::BeerDown => "Add another beer to the count.".to_string(),
            Self::BeerCount => "Show the current total of downed server beers.".to_string(),
            Self::BeerTab => "Show the current total of your downed beers.".to_string(),
            Self::Unhandled => String::new(),
        }
    }

    pub const fn variants() -> &'static [Self] {
        &[
            Self::Ping,
            Self::Help,
            Self::BeerDown,
            Self::BeerCount,
            Self::BeerTab,
        ]
    }
}

pub trait CommandHelper {
    fn command_from_str(&self) -> SlashCommand;
}

impl CommandHelper for String {
    fn command_from_str(&self) -> SlashCommand {
        match self.as_str() {
            "ping" => SlashCommand::Ping,
            "help" => SlashCommand::Help,
            "beer-down" => SlashCommand::BeerDown,
            "beer-count" => SlashCommand::BeerCount,
            "beer-tab" => SlashCommand::BeerTab,
            _ => SlashCommand::Unhandled,
        }
    }
}
