use crate::{
    application::Helper,
    models::{student::Student, topic::Topic},
    traits::gen_data_id::GenDataId,
};

#[derive(Debug, Clone)]
pub struct Group {
    id: u32,
    label: String,
    topic: Topic,
    students: Vec<Student>,
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
}

impl GenDataId<u32> for Group {
    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}
