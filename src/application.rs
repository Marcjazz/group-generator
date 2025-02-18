use std::{collections::HashSet, fmt::Debug};

use cli_table::print_stdout;
use cli_table::WithTitle;
use rand::seq::SliceRandom;

use crate::{
    data_collection::DataCollection,
    enums::labelling::Labelling,
    models::{group::Group, student::Student, topic::Topic},
    traits::{collect::Collect, gen_data_id::GenDataId},
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
            topics,
            students,
            ..
        } = &mut self.state;

        loop {
            Self::collect_gen_data(topics);

            if Self::should_break() {
                break;
            }
        }

        Helper::display(topics.iter());

        println!("Enter student names.");

        loop {
            Self::collect_gen_data(students);

            if Self::should_break() {
                break;
            }
        }

        Helper::display(students.iter());

        // Generate groups
        self.gen_groups();

        let groups = self.state.groups.clone();
        for group in groups {
            let topic = group.get_topic();
            let students = group.get_students();
            Helper::display(vec![group].iter());
            println!("{:?}", topic);
            Helper::display(students.iter());
        }
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
            let label = Self::label_gen(labelling.to_owned(), current_group_id);

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

    fn collect_gen_data<T: GenDataId<u32> + Debug + Collect>(elements: &mut Vec<T>) {
        let mut new_element = T::collect();
        new_element.set_id((elements.len() + 1) as u32);
        elements.push(new_element);
    }

    fn should_break() -> bool {
        let proceed = DataCollection::input("Do you want to continue?(yes/no)[yes]:");
        proceed.to_lowercase().eq("no")
    }

    fn label_gen(labelling: Labelling, groups_len: usize) -> String {
        match labelling {
            Labelling::Numeric => Self::num_label_gen(groups_len),
            Labelling::Alphabetic => todo!(),
            Labelling::AlphaNumeric => todo!(),
        }
    }

    fn num_label_gen(groups_len: usize) -> String {
        (groups_len + 1).to_string()
    }
}

pub struct Helper;
impl Helper {
    pub fn now_in_secs() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};

        let now = SystemTime::now();
        match now.duration_since(UNIX_EPOCH) {
            Ok(value) => value.as_secs(),
            Err(_) => {
                panic!("Time went backward!")
            }
        }
    }

    pub fn display<T: WithTitle>(table: T) {
        let _ = print_stdout(table.with_title());
    }
}
