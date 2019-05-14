mod workspace;

use clap::{App, Arg, SubCommand};
use code_analysis::SourceRootId;
use vue_analysis::{Analysis, Config};
use std::{fs, io, path::PathBuf};

fn main() -> Result<(), io::Error> {
    let args = App::new("vue_analyzer")
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("check")
                .arg(
                    Arg::with_name("config")
                        .long("config")
                        .takes_value(true)
                        .value_name("CONFIG_FILE")
                )
                // .arg(
                //     Arg::with_name("file")
                //         .long("file")
                //         .takes_value(true)
                //         .value_name("TARGET_FILE")
                // )
                .arg(Arg::with_name("main").required(true))
        )
        .subcommand(
            SubCommand::with_name("parse")
                .arg(Arg::with_name("script").long("script"))
                .arg(Arg::with_name("file").required(true))
        )
        .get_matches();
    match args.subcommand() {
        ("check", Some(args)) => {
            // Load "single-file" project
            /*
            let filetext = fs::read_to_string(filename)?;
            let (mut analysis, file_id) = Analysis::from_single_file(filename.into(), filetext);
            */

            // Load project
            let entrypoint = PathBuf::from(args.value_of("main").unwrap());
            let (mut analysis, vfs) = workspace::load(entrypoint.clone());
            let root_id = match vfs.path2file(&entrypoint) {
                Some(id) => SourceRootId(id.0),
                None => {
                    eprintln!("error(usage): could not find file '{}'", entrypoint.display());
                    std::process::exit(1);
                }
            };

            // Load configuration
            if let Some(config_path) = args.value_of("config") {
                let config_text = fs::read_to_string(config_path)?;
                let config: Config = if config_path.ends_with("json") {
                    serde_json::from_str(&config_text).unwrap()
                } else {
                    toml::from_str(&config_text).unwrap()
                };
                analysis.set_config(config);
            } else {
                analysis.set_config(Config::default());
            }

            // Run diagnostics
            let mut total_errors = 0;
            for (path, file_id) in analysis.files(root_id) {
                match path.extension() {
                    Some("js") | Some("ts") | Some("vue") => {
                        let diagnostics = analysis.diagnostics(file_id.into());
                        for line in diagnostics {
                            if line.starts_with("error") {
                                total_errors += 1;
                            }
                            eprintln!("{}", line);
                        }
                    },
                    _ => continue,
                };
            }
            eprintln!("info: found {} error(s)", total_errors);
            std::process::exit(if total_errors > 0 { 1 } else { 0 });
        }
        ("parse", Some(args)) => {
            let filename = args.value_of("file").unwrap();
            let filetext = fs::read_to_string(filename)?;
            let (analysis, file_id) = Analysis::from_single_file(filename.into(), filetext);
            println!("{}", analysis.file_syntax_tree(file_id.into(), args.is_present("script")))
        }
        _ => (),
    }
    Ok(())
}
