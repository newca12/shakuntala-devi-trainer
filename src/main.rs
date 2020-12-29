mod cli;

fn main() {
    let args = cli::parse_cli();
    match args {
        false => {
            println!("gui not yet implemented");
        }
        true => {
            cli::run_cli();
        }
    };
}
