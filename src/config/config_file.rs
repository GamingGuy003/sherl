use std::fmt::Display;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConfigFile {
    pub prompt: Prompt,
    pub verbose: bool,
}

#[derive(Deserialize)]
pub struct Prompt {
    pub format: String,
}

pub enum PromptElements {
    Username,
    CurrentWorkingDirectory,
}

pub enum Colors {
    Black,       // 30
    Red,         // 31
    Green,       // 32
    Brown,       // 33
    Blue,        // 34
    Purple,      // 35
    Cyan,        // 36
    LightGray,   // 37
    DarkGray,    // 1;30
    LightRed,    // 1;31
    LightGreen,  // 1;32
    Yellow,      // 1;33
    LightBlue,   // 1;34
    LightPurple, // 1;35
    LightCyan,   // 1;36
    WHite,       // 1;37
}

pub enum Style {
    Normal,        // 0
    Bold,          // 1
    Weak,          // 2
    Underline,     // 4
    Reverse,       // 7
    StrikeThrough, // 9
}

impl Display for Prompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
