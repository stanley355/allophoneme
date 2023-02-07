
mod cli;

fn main() {
    cli::Cli::welcome_text();
    cli::Cli::welcome_input();
    // let args: Vec<String> = env::args().collect();

    // let query = &args[0];
    // println!("{}", query);

}
