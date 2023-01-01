use gui::run_gui;

mod cli;
mod gui;

fn main() {
    let args = cli::parse_cli();
    match args {
        false => {
            run_gui();
        }
        true => {
            cli::run_cli();
        }
    };
}
