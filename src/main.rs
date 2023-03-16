mod cli;
mod excel;
mod levenshtein;
mod constant;
mod doodle;
mod doodle_similarity;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() -> glib::ExitCode {
    // cli::Cli::start_menu();
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .build();

        // Show the window.
        window.show();
    });

    app.run()
}