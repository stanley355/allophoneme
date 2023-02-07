
mod cli;

fn main() {
    cli::Cli::welcome_text();
    cli::Cli::welcome_input();

    // let file_path = "../poem.txt";
    // let contents = fs::read_to_string(file_path)
    //     .expect("Should have been able to read the file");

    // println!("With text:\n{contents}");
}
