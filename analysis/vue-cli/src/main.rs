use clap::{App, Arg, SubCommand};
use vue_analysis::{Analysis, Config};
use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let args = App::new("vue_analyzer")
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("lint")
                .arg(
                    Arg::with_name("config")
                        .long("config")
                        .takes_value(true)
                        .value_name("FILE")
                )
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
            let (mut analysis, file_id) = Analysis::from_single_file(filename.into(), filetext);
            if let Some(config_path) = args.value_of("config") {
                let config_text = fs::read_to_string(config_path)?;
                let config: Config = if config_path.ends_with("json") {
                    serde_json::from_str(&config_text).unwrap()
                } else {
                    toml::from_str(&config_text).unwrap()
                };
                analysis.set_config(config);
            }
            let diagnostics = analysis.diagnostics(file_id.into());
            let mut total_errors = 0;
            for line in diagnostics {
                if line.starts_with("error") {
                    total_errors += 1;
                }
                eprintln!("{}", line);
            }
            eprintln!("info: found {} error(s)", total_errors);
            std::process::exit(if total_errors > 0 { 1 } else { 0 });
        }
        ("parse", Some(args)) => {
            let filename = args.value_of("file").unwrap();
            let filetext = fs::read_to_string(filename)?;
            let (analysis, file_id) = Analysis::from_single_file(filename.into(), filetext);
            println!("{}", analysis.file_syntax_tree(file_id.into()))
        }
        _ => (),
    }
    Ok(())
}
