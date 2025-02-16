use std::rc::Rc;

use crate::models::{student::Student, topic::Topic};

#[derive(Debug)]
pub struct Group {
    id: u32,
    label: String,
    topic: Rc<Topic>,
    students: Vec<Rc<Student>>,
}

impl Group {
    pub fn from(label: String, topic: Rc<Topic>) -> Self {
        Self {
            id: 0,
            label,
            topic,
            students: Vec::new(),
        }
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = id
    }
}
