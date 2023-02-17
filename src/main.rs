mod cli;
mod excel;
mod database;
use tokio::runtime::Runtime;

fn main() {
    cli::Cli::start_menu();
    // Create a new Tokio runtime
    // let rt = Runtime::new().unwrap();

    // Run your async task within the Tokio runtime
    // rt.block_on(async {
    //     database::Database::create_pg_pool().await;
    // });
}
