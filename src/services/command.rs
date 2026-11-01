pub enum Command {
    Ping,
    Help,
    BeerDown,
    BeerCount,
    BeerTab,
    // unhandled
    Unhandled,
}

impl Command {
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
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
            Self::Unhandled,
        ]
    }
}

pub trait CommandHelper {
    fn command_from_str(&self) -> Command;
}

impl CommandHelper for String {
    fn command_from_str(&self) -> Command {
        match self.as_str() {
            "ping" => Command::Ping,
            "help" => Command::Help,
            "beer-down" => Command::BeerDown,
            "beer-count" => Command::BeerCount,
            "beer-tab" => Command::BeerTab,
            _ => Command::Unhandled,
        }
    }
}
