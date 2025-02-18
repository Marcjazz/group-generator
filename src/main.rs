use application::Application;

fn main() {
    let mut app = Application::new();
    app.run();
}

mod data_collection;
mod enums;
mod models;
mod application;
mod traits;