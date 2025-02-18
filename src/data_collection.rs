use std::io;

pub struct DataCollection;
impl DataCollection {
    pub fn input(prompt: &str) -> String {
        println!("{prompt}");

        let mut new_input = String::new();

        io::stdin()
            .read_line(&mut new_input)
            .expect("Failed to read line");

        new_input.trim().to_owned()
    }
}
