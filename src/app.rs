use core::panic;
use serde::{Deserialize, Serialize};
use std::{error, path::Path};
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhysicalCharacteristics {
    pub height: f32,
    pub weight: f32,
    pub eye_color: String,
    pub hair_color: String,
    pub race: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MentalCharacteristics {
    pub alignment: String,
    pub disorders: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub name: String,
    pub age: u8,
    pub color: u32,
    pub physical_characteristics: PhysicalCharacteristics,
    pub mental_characteristics: MentalCharacteristics,
    pub story_appearances: Vec<String>,
    pub bonds: String,
    pub description: String,
    pub species: String,
}
/// Application.
#[derive(Debug, Clone)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Current File
    pub curr: u8,
    /// Paths
    pub paths: Vec<String>,
    pub curr_char: Character,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(character_path: String) -> Self {
        if Path::new(&character_path).is_dir() {
            let path_reader = std::fs::read_dir(character_path).unwrap();

            let mut strings: Vec<String> = Vec::new();
            for path in path_reader {
                strings.push(path.unwrap().file_name().to_string_lossy().to_string())
            }

            let current_file =
                std::fs::read_to_string(format!("./characters/{}", strings[0])).unwrap();

            let character: Character = serde_json::from_str(&current_file.as_str().trim())
                .expect(format!("{current_file}").as_str());
            Self {
                running: true,
                curr: 0,
                paths: strings,
                curr_char: character,
            }
        } else {
            panic!["Invalid character directory!"]
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_curr(&mut self) {
        self.curr = (self.curr + self.paths.len() as u8 - 1) % self.paths.len() as u8;
        self.parse_character()
    }

    pub fn decrement_curr(&mut self) {
        self.curr = (self.curr + 1) % self.paths.len() as u8;
        self.parse_character()
    }
    pub fn parse_character(&mut self) {
        let current_file = std::fs::read_to_string(format!(
            "./characters/{}",
            self.paths[self.curr as usize].clone()
        ))
        .unwrap();

        let character: Character = serde_json::from_str(&current_file.as_str().trim())
            .expect(format!("{current_file}").as_str());
        self.curr_char = character;
    }
}
