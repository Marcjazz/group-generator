use application::Application;

fn main() {
    let mut app = Application::new();
    app.run();
}

mod helper;
mod enums;
mod models;
mod application;
mod traits;