mod command;
mod issue_link;

fn main() {
    if let Err(e) = command::run() {
        eprintln!("{}", e);

        std::process::exit(1)
    };
}
