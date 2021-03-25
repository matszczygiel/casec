mod converter;

use std::{
    error::Error,
    io::{Read, Write},
    path::PathBuf,
    str::FromStr,
};

use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone, Copy)]
pub enum Case {
    Snake,
    Camel,
}

impl FromStr for Case {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "snake" | "s" => Ok(Self::Snake),
            "camel" | "c" => Ok(Self::Camel),
            _ => Err("Invalid case name"),
        }
    }
}

/// Search for identifiers matching the pattern in the text and change its case.
#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Cli {
    /// The list of patterns to look for
    #[structopt(long, short)]
    patterns: Vec<String>,
    /// Case to change identifier for
    #[structopt(long, short)]
    case: Case,
    /// Output to file, by default output is written to stdout
    #[structopt(long, short, parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();

    let result = {
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input).unwrap();

        converter::convert(&input, args.patterns, args.case)?
    };

    if let Some(output) = args.output {
        let mut file = std::fs::File::create(output).unwrap();
        file.write_all(&result.into_bytes()).unwrap();
    } else {
        std::io::stdout().write_all(&result.into_bytes()).unwrap();
    }

    Ok(())
}
