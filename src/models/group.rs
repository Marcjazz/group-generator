use cli_table::{format::Justify, Table};

use crate::{
    application::Helper,
    models::{student::Student, topic::Topic},
    traits::gen_data_id::GenDataId,
};

#[derive(Debug, Clone, Table)]
pub struct Group {
    #[table(title = "ID", justify = "Justify::Right")]
    id: u32,

    #[table(title = "Label")]
    label: String,

    #[table(skip)]
    topic: Topic,

    #[table(skip)]
    students: Vec<Student>,

    #[table(title = "Generated At")]
    generated_at: u64,
}

impl Group {
    pub fn from(label: String, topic: Topic, students: Vec<Student>) -> Self {
        Self {
            id: 0,
            label,
            topic,
            students,
            generated_at: Helper::now_in_secs(),
        }
    }

    pub fn get_students(&self) -> Vec<Student> {
        self.students.clone()
    }

    pub fn get_topic(&self) -> Topic {
        self.topic.clone()
    }
}

impl GenDataId<u32> for Group {
    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}
