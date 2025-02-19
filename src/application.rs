use cli_table::{Cell, Table};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fmt::Debug, io::Error};

use crate::{
    enums::labelling::Labelling,
    file_manager::FileManager,
    helper::{CollectDataHelper, DisplayHelper, Helper, LabellingHelper},
    models::{group::Group, student::Student, topic::Topic},
    traits::gen_data_id::GenDataId,
};

#[derive(Debug, Serialize, Deserialize)]
struct AppState {
    labelling: Labelling,
    topics: Vec<Topic>,
    students: Vec<Student>,
    groups: Vec<Group>,
}

#[derive(Debug)]
pub struct Application {
    state: AppState,
    file_manager: FileManager,
}

impl Application {
    pub fn new(app_name: &str) -> Self {
        Self {
            state: AppState {
                labelling: Labelling::Numeric,
                groups: Vec::new(),
                students: Vec::new(),
                topics: Vec::new(),
            },
            file_manager: FileManager::new(app_name),
        }
    }

    pub fn launch(&mut self) {
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

        let should_save =
            CollectDataHelper::read_input("Do wich to save application state?(yes/no)[no]:")
                .to_lowercase()
                .eq("yes");

        // Save application state
        if should_save {
            self.save_app_state();
        }
    }

    /// Persist application data
    pub fn save_app_state(&self) {
        let filename = format!("grp_{}.bin", Helper::now_in_secs());
        match self.file_manager.save_to_file(&self.state, filename.as_str()) {
            Ok(_) => {}
            Err(e) => eprintln!("Could not save application state: {e}"),
        }
    }

    /// Starts application by loading data from file system
    pub fn start(&mut self) -> std::io::Result<()> {
        let app_state_files = self.file_manager.get_saved_files()?;
        let table = app_state_files
            .iter()
            .enumerate()
            .map(|(i, file)| vec![i.cell(), file.cell()])
            .table()
            .title(vec!["ID", "Saving Name"]);
        println!("{}", table.display().unwrap());

        let id: usize = CollectDataHelper::read_input("Enter Saving ID:")
            .parse()
            .expect("Invalid input");

        if id >= app_state_files.len() {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Index out of bound.",
            ));
        }

        let filename = &app_state_files[id];
        self.state = self.file_manager.load_from_file(filename.as_str())?;

        DisplayHelper::display(self.state.groups.iter());

        Ok(())
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
