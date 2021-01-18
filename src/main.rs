mod cli;
mod gui;

fn main() {
    let args = cli::parse_cli();
    match args {
        false => {
            gui::run_gui();
        }
        true => {
            cli::run_cli();
        }
    };
}
