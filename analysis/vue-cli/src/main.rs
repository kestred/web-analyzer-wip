use clap::{App, Arg, SubCommand};
use vue_analysis::Analysis;

fn main() {
    let args = App::new("vue_analyzer")
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("lint")
                .arg(Arg::with_name("file").required(true))
        )
        .get_matches();
    match args.subcommand() {
        ("lint", Some(lint_args)) => {
            let filename = lint_args.value_of("file").unwrap();
            let (analysis, file_id) = Analysis::from_single_file(filename.into());
            let diagnostics = analysis.diagnostics(file_id.into());
            if diagnostics.is_empty() {
                println!("Nothing to report!");
            }


            println!("Found {} issues:", diagnostics.len());
            for line in diagnostics {
                println!("  {}", line);
            }
        }
        _ => (),
    }
}
