use std::{
    io::{Read, Write},
    path::PathBuf,
    str::{Chars, FromStr},
};

use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone, Copy)]
pub enum Case {
    Snake,
    Cammel,
}

impl FromStr for Case {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "snake" | "s" => Ok(Self::Snake),
            "cammel" | "c" => Ok(Self::Cammel),
            _ => Err("Invalid case name"),
        }
    }
}

/// Search for a pattern in stdin
#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Cli {
    /// The patterns to look for
    #[structopt(long, short)]
    identifiers: Vec<String>,
    #[structopt(long, short)]
    case: Case,
    #[structopt(long, short, parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() {
    let args = Cli::from_args();

    let res = {
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input).unwrap();

        convert(input, args.identifiers, args.case)
    };

    if let Some(output) = args.output {
        let mut file = std::fs::File::create(output).unwrap();
        file.write_all(&res.into_bytes()).unwrap();
    } else {
        std::io::stdout().write_all(&res.into_bytes()).unwrap();
    }
}

pub fn convert<S>(input: String, patters: impl IntoIterator<Item = S>, case: Case) -> String
where
    S: AsRef<str>,
{
    let replacer = Replacer { case };
    let mut res = input;

    for pattern in patters {
        let rg = regex::Regex::new(pattern.as_ref()).unwrap();
        let cow = rg.replace_all(&res, replacer);
        res = cow.to_string();
    }

    res.to_string()
}

#[derive(Debug, Clone, Copy)]
struct Replacer {
    case: Case,
}

impl Replacer {
    fn append_cammel(&self, chars: Chars, dst: &mut String) {
        let mut underscore = false;
        for c in chars {
            match c {
                '_' | '-' => underscore = true,
                _ => {
                    if underscore {
                        underscore = false;
                        dst.push(c.to_ascii_uppercase());
                    } else {
                        dst.push(c);
                    }
                }
            }
        }
    }

    fn append_snake(&self, chars: Chars, dst: &mut String) {
        let mut whitespace = true;

        for c in chars {
            if c.is_uppercase() {
                if !whitespace {
                    dst.push('_');
                }
            }

            dst.push(c.to_ascii_lowercase());
            whitespace = c.is_whitespace();
        }
    }
}

impl regex::Replacer for Replacer {
    fn replace_append(&mut self, caps: &regex::Captures, dst: &mut String) {
        let chars = caps.get(0).unwrap().as_str().chars();

        match self.case {
            Case::Snake => self.append_snake(chars, dst),
            Case::Cammel => self.append_cammel(chars, dst),
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    const INPUT: &'static str =
        "word CammelCase Word cammelCase CAMMELCASE anotherOfThisKind snake_case _snake_case";

    #[test]
    fn cammel_to_snake() {
        let res = convert(INPUT.to_string(), &["cammelCase"], Case::Snake);
        assert_eq!(
            res,
            "word CammelCase Word cammel_case CAMMELCASE anotherOfThisKind snake_case _snake_case"
        );

        let res = convert(INPUT.to_string(), &[".ammelCase"], Case::Snake);
        assert_eq!(
            res,
            "word cammel_case Word cammel_case CAMMELCASE anotherOfThisKind snake_case _snake_case"
        );
    }

    #[test]
    fn snake_to_cammel() {
        let res = convert(INPUT.to_string(), &["snake_case"], Case::Cammel);
        assert_eq!(
            res,
            "word CammelCase Word cammelCase CAMMELCASE anotherOfThisKind snakeCase _snakeCase"
        );

        let res = convert(INPUT.to_string(), &[r".snake_case"], Case::Cammel);
        assert_eq!(
            res,
            "word CammelCase Word cammelCase CAMMELCASE anotherOfThisKind snakeCase SnakeCase"
        );
    }
}
