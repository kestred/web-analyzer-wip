use clap::{App, Arg, SubCommand};
use vue_analysis::Analysis;
use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let args = App::new("vue_analyzer")
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("lint")
                .arg(Arg::with_name("file").required(true))
        )
        .subcommand(
            SubCommand::with_name("parse")
                .arg(Arg::with_name("file").required(true))
        )
        .get_matches();
    match args.subcommand() {
        ("lint", Some(args)) => {
            let filename = args.value_of("file").unwrap();
            let filetext = fs::read_to_string(filename)?;
            let (analysis, file_id) = Analysis::from_single_file(filename.into(), filetext);
            let diagnostics = analysis.diagnostics(file_id.into());
            if diagnostics.is_empty() {
                println!("Nothing to report!");
                std::process::exit(0);
            }

            let total = diagnostics.len();
            for line in diagnostics {
                eprintln!("{}", line);
            }
            eprintln!("(total: {})", total);
            std::process::exit(1);
        }
        ("parse", Some(args)) => {
            let filename = args.value_of("file").unwrap();
            let filetext = fs::read_to_string(filename)?;
            let (analysis, file_id) = Analysis::from_single_file(filename.into(), filetext);
            println!("{}", analysis.debug_syntax_tree(file_id.into()))
        }
        _ => (),
    }
    Ok(())
}
