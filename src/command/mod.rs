use clap::*;
mod dialogure;
mod template;

type App = clap::App<'static, 'static>;

pub fn run() {
    let matches = build_app().get_matches();

    if matches.is_present("file") {
        return;
    }

    match matches.subcommand() {
        ("dialogure", _) => (),
        ("template", _) => (),
        _ => build_app().print_help().expect("failed to print_help"),
    }
}

fn build_app() -> App {
    clap::App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("file"))
        .subcommand(SubCommand::with_name("dialogure"))
        .subcommand(SubCommand::with_name("template"))
}
