use crate::{
    enums::difficulty::Difficulty,
    helper::CollectDataHelper,
    traits::{collect::Collect, gen_data_id::GenDataId},
};
use cli_table::{format::Justify, Table};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Table, Serialize, Deserialize)]
pub struct Topic {
    #[table(title = "ID", justify = "Justify::Right")]
    id: u32,

    #[table(title = "Title")]
    title: String,

    #[table(title = "Difficulty")]
    difficulty: Difficulty,

    #[table(title = "description")]
    description: String,
}

impl Topic {
    pub fn new() -> Self {
        Self {
            id: 0,
            title: String::new(),
            difficulty: Difficulty::Easy,
            description: String::new(),
        }
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title
    }

    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_difficulty(&self) -> Difficulty {
        self.difficulty.clone()
    }
}

impl Collect for Topic {
    fn collect() -> Self {
        let mut topic = Self::new();

        topic.title = CollectDataHelper::read_input("Enter topic title:");

        let difficulty = CollectDataHelper::read_input("Enter topic difficulty:");
        topic.difficulty = Difficulty::from(difficulty.as_str());

        topic.description = CollectDataHelper::read_input("Enter topic description:");

        topic
    }
}

impl GenDataId<u32> for Topic {
    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}

impl Display for Topic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Topic(ID={}, Title={}, Difficulty={})\n\n{}",
            self.id,
            self.title.bold().green(),
            self.difficulty.to_string().bold().green(),
            format!("Description: {}", self.description).italic()
        )
    }
}
