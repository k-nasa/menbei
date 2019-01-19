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
        ("dialogure", _) => dialogure::exec(),
        ("template", _) => template::exec(),
        _ => build_app().print_help().expect("failed to print_help"),
    }
}

fn build_app() -> App {
    clap::App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("file").help("issue link definition file"))
        .subcommand(SubCommand::with_name("dialogure").about("Create an issue link interactively"))
        .subcommand(
            SubCommand::with_name("template")
                .about("Output a template file for creating an issue link")
                .arg(
                    Arg::with_name("name")
                        .help("File name to output")
                        .required(true),
                ),
        )
}
