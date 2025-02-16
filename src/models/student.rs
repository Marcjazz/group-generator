use crate::{data_collection::DataCollection, traits::collect::Collect};

#[derive(Debug)]
pub struct Student {
    id: u32,
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

    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Collect for Student {
    fn collect() -> Self {
        let prompt = Some("Enter student name:".to_string());
        let name = DataCollection::input(prompt);

        Self::from(name)
    }
}
