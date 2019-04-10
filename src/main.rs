extern crate clap;

use clap::{App, Arg, SubCommand};
use std::error::Error;

mod scoring_rule;

fn main() -> Result<(), Box<dyn Error>> {
    let app = get_app();

    if let Some(m) = app.get_matches().subcommand_matches("add-rule") {
        let rule = scoring_rule::add(m.value_of("name").unwrap());
        match rule {
            Ok(r) => {
                println!("Saved the rule: {:#?}", r);
                return Ok(());
            }
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

fn get_app<'a, 'b>() -> App<'a, 'b> {
    App::new("mlbh2h")
        .version("0.1.2")
        .author("Soo Philip Jason Kim <philipjkim@gmail.com>")
        .about(
            "This app Shows Yahoo! Baseball Head-to-Head fantasy points by your scoring settings.",
        )
        .subcommand(
            SubCommand::with_name("add-rule")
                .about("adds a fantasy point scoring rule")
                .arg(
                    Arg::with_name("name")
                        .short("n")
                        .help("set name of the new scoring rule")
                        .takes_value(true)
                        .required(true),
                ),
        )
}
