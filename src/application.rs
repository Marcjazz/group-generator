use std::rc::Rc;

use crate::{enums::labelling::Labelling, models::{group::Group, topic::Topic}, traits::collect::Collect};

#[derive(Debug)]
struct AppState {
    date: u64,
    labelling: Labelling,
    groups: Rc<Vec<Group>>,
}
pub struct Application {
    state: AppState,
}

impl Application {
    pub fn new() -> Self {
        Self {
            state: AppState {
                date: 0,
                labelling: Labelling::Numeric,
                groups: Rc::new(Vec::new()),
            },
        }
    }

    pub fn run(&self) {
        println!("Enter the topics.\nType 'done' when completed.");
        
        loop {
            let topic = Topic::collect();
            
        }
    }

    fn label_gen(&self) -> String {
        match self.state.labelling {
            Labelling::Numeric => Self::num_label_gen(self),
            Labelling::Alphabetic => todo!(),
            Labelling::AlphaNumeric => todo!(),
        }
    }

    fn num_label_gen(&self) -> String {
        (self.state.groups.len() + 1).to_string()
    }
}
