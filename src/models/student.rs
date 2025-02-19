use std::fmt::Display;

use crate::{
    helper::CollectDataHelper,
    traits::{collect::Collect, gen_data_id::GenDataId},
};
use cli_table::{format::Justify, Table};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Table, Serialize, Deserialize)]
pub struct Student {
    #[table(title = "ID", justify = "Justify::Right")]
    id: u32,

    #[table(title = "Name")]
    name: String,
}

impl Student {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: String::new(),
        }
    }

    pub fn from(name: String) -> Self {
        Self { id: 0, name }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Collect for Student {
    fn collect() -> Self {
        let name = CollectDataHelper::read_input("Enter student name:");

        Self::from(name)
    }
}

impl GenDataId<u32> for Student {
    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}

impl Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Student(ID={}, Name={})", self.id, self.name)
    }
}
