mod cli;
mod excel;
mod database;
use tokio::runtime::Runtime;

fn main() {
    cli::Cli::start_menu();
}
