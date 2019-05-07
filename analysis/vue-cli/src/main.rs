use clap::{App, Arg, SubCommand};
// use grammar_utils::ast;
// use html_grammar::ast as html;
// use javascript_grammar::ast as javascript;
use std::{fs, io::{self, Read}, path::Path};

fn main() -> Result<(), io::Error> {
    let args = App::new("vue_analyzer")
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("lint"));
    match args.subcommand() {
        ("lint", _) => {
            let input = read_stdin()?;
        }
    }
    Ok(())
}

fn read_stdin() -> Result<String, io::Error> {
    let mut buff = String::new();
    io::stdin().read_to_string(&mut buff)?;
    Ok(buff)
}
