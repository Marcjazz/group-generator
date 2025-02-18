use std::{
    fmt::{Debug, Display},
    io,
};

use cli_table::{print_stdout, Table, WithTitle};

use crate::{
    enums::labelling::Labelling,
    traits::{collect::Collect, gen_data_id::GenDataId},
};

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
}

pub struct LabellingHelper;
impl LabellingHelper {
    pub fn label_gen(labelling: Labelling, groups_len: usize) -> String {
        match labelling {
            Labelling::Numeric => Self::num_label_gen(groups_len),
            Labelling::Alphabetic => todo!(),
            Labelling::AlphaNumeric => todo!(),
        }
    }

    pub fn num_label_gen(groups_len: usize) -> String {
        (groups_len + 1).to_string()
    }
}

pub struct DisplayHelper;
impl DisplayHelper {
    pub fn display<T: Table + WithTitle>(table: T) {
        match print_stdout(table.with_title()) {
            Ok(_) => {}
            Err(e) => eprintln!("{e}"),
        };
    }

    pub fn stringify<T: Display>(elements: &Vec<T>, discriminator: &str) -> String {
        elements
            .iter()
            .map(|s| s.to_string()) // Uses `Display` implementation
            .collect::<Vec<_>>()
            .join(discriminator)
    }
}

pub struct CollectDataHelper;
impl CollectDataHelper {
    pub fn read_input(prompt: &str) -> String {
        println!("{prompt}");

        let mut new_input = String::new();

        io::stdin()
            .read_line(&mut new_input)
            .expect("Failed to read line");

        new_input.trim().to_owned()
    }

    pub fn collect_gen_data<T: GenDataId<u32> + Debug + Collect>(elements: &mut Vec<T>) {
        let mut new_element = T::collect();
        new_element.set_id((elements.len() + 1) as u32);
        elements.push(new_element);
    }

    pub fn should_break() -> bool {
        let proceed = Self::read_input("Do you want to continue?(yes/no)[yes]:");
        proceed.to_lowercase().eq("no")
    }
}
