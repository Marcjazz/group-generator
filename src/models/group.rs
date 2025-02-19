use cli_table::{format::Justify, Table};
use serde::{Deserialize, Serialize};

use crate::{
    helper::{DisplayHelper, Helper},
    models::{student::Student, topic::Topic},
    traits::gen_data_id::GenDataId,
};

#[derive(Debug, Clone, Table, Serialize, Deserialize)]
pub struct Group {
    #[table(title = "ID", justify = "Justify::Right")]
    id: u32,

    #[table(title = "Label")]
    label: String,

    #[table(title = "Topic")]
    topic: Topic,

    #[table(skip)]
    students: Vec<Student>,

    #[table(title = "Generated At")]
    generated_at: u64,

    #[table(title = "Students")]
    students_display: String, // Derived field
}

impl Group {
    pub fn from(label: String, topic: Topic, students: Vec<Student>) -> Self {
        let students_display = DisplayHelper::stringify(&students, "\n");

        Self {
            id: 0,
            label,
            topic,
            students,
            students_display,
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
