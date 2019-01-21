mod command;
mod issue_link;

fn main() {
    match command::run() {
        Err(e) => eprintln!("{}", e),
        Ok(_) => (),
    };
}
