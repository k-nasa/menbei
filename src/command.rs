use clap::*;

type App = clap::App<'static, 'static>;

pub fn run() {
    let mut app = build_app();

    app.print_help().expect("failed to print_help");
}

fn build_app() -> App {
    clap::App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("file"))
        .subcommand(SubCommand::with_name("dialogure"))
        .subcommand(SubCommand::with_name("template"))
}
