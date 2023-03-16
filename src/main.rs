// mod cli;
// mod excel;
// mod levenshtein;
// mod constant;
// mod doodle;
// mod doodle_similarity;

use gtk::prelude::*;
use gtk::{glib, Application};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() -> glib::ExitCode {
    // cli::Cli::start_menu();
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Run the application
    app.run()
}
