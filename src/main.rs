use gui::app;

mod cli;
mod gui;

fn main() {
    let args = cli::parse_cli();
    match args {
        false => {
            app::run_gui();
        }
        true => {
            cli::run_cli();
        }
    };
}
