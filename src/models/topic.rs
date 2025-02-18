use std::fmt::Display;

use crate::{
    data_collection::DataCollection,
    enums::difficulty::Difficulty,
    traits::{collect::Collect, gen_data_id::GenDataId},
};
use cli_table::{format::Justify, Table};

#[derive(Debug, Clone, Table)]
pub struct Topic {
    #[table(title = "ID", justify = "Justify::Right")]
    id: u32,

    #[table(title = "Title")]
    title: String,

    #[table(title = "Difficulty")]
    difficulty: Difficulty,
}

impl Topic {
    pub fn new() -> Self {
        Self {
            id: 0,
            title: String::from(""),
            difficulty: Difficulty::Easy,
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

        topic.title = DataCollection::input("Enter topic title:");

        let difficulty = DataCollection::input("Enter topic difficulty:");
        topic.difficulty = Difficulty::from(difficulty.as_str());

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
            "Topic(ID={}, Title={}, Difficulty={:?})",
            self.id, self.title, self.difficulty
        )
    }
}
