use rand::seq::SliceRandom;
use std::{collections::HashSet, fmt::Debug};

use crate::{
    enums::labelling::Labelling,
    helper::{CollectDataHelper, DisplayHelper, LabellingHelper},
    models::{group::Group, student::Student, topic::Topic},
    traits::gen_data_id::GenDataId,
};

#[derive(Debug)]
struct AppState {
    labelling: Labelling,
    topics: Vec<Topic>,
    students: Vec<Student>,
    groups: Vec<Group>,
}

pub struct Application {
    state: AppState,
}

impl Application {
    pub fn new() -> Self {
        Self {
            state: AppState {
                labelling: Labelling::Numeric,
                groups: Vec::new(),
                students: Vec::new(),
                topics: Vec::new(),
            },
        }
    }

    pub fn run(&mut self) {
        println!("Enter new topics.");

        let AppState {
            topics, students, ..
        } = &mut self.state;

        loop {
            CollectDataHelper::collect_gen_data(topics);

            if CollectDataHelper::should_break() {
                break;
            }
        }

        println!("Enter student names.");

        loop {
            CollectDataHelper::collect_gen_data(students);

            if CollectDataHelper::should_break() {
                break;
            }
        }

        DisplayHelper::display(topics.iter());
        DisplayHelper::display(students.iter());


        // Generate groups
        self.gen_groups();

        DisplayHelper::display(self.state.groups.iter());
    }

    fn gen_groups(&mut self) {
        use rand::rng;

        let AppState {
            topics,
            students,
            labelling,
            ..
        } = &self.state;

        let mut new_groups = Vec::new();
        let nbr_of_members = students.len() / topics.len();
        let mut assigned_student_ids = HashSet::<u32>::new();

        for topic in topics {
            let current_group_id = new_groups.len() + 1;
            let label = LabellingHelper::label_gen(labelling.to_owned(), current_group_id);

            let mut students: Vec<Student> = self
                .state
                .students
                .iter()
                .filter(|s| !assigned_student_ids.contains(&s.get_id()))
                .map(|s| s.to_owned())
                .collect();

            // shuffle array of students
            let mut rng_gen = rng();
            students.shuffle(&mut rng_gen);

            // select group members safely
            let grp_members = students
                .iter()
                .take(nbr_of_members)
                .map(|student| {
                    assigned_student_ids.insert(student.get_id());
                    student
                })
                .cloned()
                .collect();

            let mut new_group = Group::from(label, topic.to_owned(), grp_members);
            new_group.set_id(current_group_id as u32);
            new_groups.push(new_group);
        }

        self.state.groups.append(&mut new_groups.clone());
    }
}
