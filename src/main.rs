use application::Application;

fn main() {
    let mut app = Application::new("Smart Grp");

    // start for saved data or launch
    app.start().unwrap_or_else(|_| app.launch());
}

mod application;
mod enums;
mod file_manager;
mod helper;
mod models;
mod traits;
