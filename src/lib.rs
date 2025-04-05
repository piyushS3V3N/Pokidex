pub mod actions;
pub mod configs;
pub mod events;
pub mod file_handler;
pub mod widget;

use configs::Configs;
use serde::{Deserialize, Serialize};
use std::{error::Error, io::Stdout};
use tui::{
    backend::CrosstermBackend,
    style::{Modifier, Style},
    Frame, Terminal,
};

pub type DynResult = Result<(), Box<dyn Error>>;
pub type CrossTerminal = Terminal<CrosstermBackend<Stdout>>;
pub type TerminalFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

/// Represent a task
#[derive(Serialize, Deserialize, Clone)]
pub struct Pokidex {
    pub title: String,
    pub completed: bool,
}

impl Pokidex {
    pub fn new(title: String) -> Self {
        Self {
            title,
            completed: false,
        }
    }
}

/// Represent a list of tasks
#[derive(Serialize, Deserialize, Default)]
pub struct PokidexList {
    pub pokientrys: Vec<Pokidex>,
}

impl PokidexList {
    pub fn new(pokientrys: &[Pokidex]) -> Self {
        Self {
            pokientrys: pokientrys.to_vec(),
        }
    }
}

/// Possible Input field states
pub enum InputMode {
    /// Browsing pokientrys
    Normal,
    /// Adding a new pokientry
    Adding,
}

/// Application state
pub struct App {
    /// New pokientry input value
    pub input: String,
    /// Current input mode
    pub input_mode: InputMode,
    /// List of all pokientrys
    pub pokientrys: Vec<Pokidex>,
    /// Should be true when application wants to exit
    pub should_exit: bool,
    /// Current selected pokientry
    pub selected_pokientry: Option<usize>,
    /// Application Configs
    pub configs: Configs,
}

impl App {
    pub fn new(pokientrys: &[Pokidex], configs: Configs) -> Self {
        Self {
            pokientrys: pokientrys.to_vec(),
            selected_pokientry: Some(0),
            input: String::new(),
            input_mode: InputMode::Normal,
            should_exit: false,
            configs,
        }
    }

    pub fn default_style(&self) -> Style {
        Style::default()
            .fg(self.configs.colors.foreground)
            .bg(self.configs.colors.background)
    }

    pub fn selection_style(&self) -> Style {
        self.default_style()
            .fg(self.configs.colors.selection_fg)
            .bg(self.configs.colors.selection_bg)
    }

    pub fn check_sign_style(&self, selected: bool) -> Style {
        if selected {
            self.selection_style().fg(self.configs.colors.check_sign)
        } else {
            self.default_style().fg(self.configs.colors.check_sign)
        }
    }

    pub fn checked_pokientry_style(&self, selected: bool) -> Style {
        if selected {
            self.selection_style().add_modifier(Modifier::CROSSED_OUT)
        } else {
            self.default_style().add_modifier(Modifier::CROSSED_OUT)
        }
    }
}
