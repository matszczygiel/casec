use std::{error::Error, str::Chars};

use regex::RegexSet;

use crate::Case;

pub fn convert<S>(
    input: &str,
    patters: impl IntoIterator<Item = S>,
    case: Case,
) -> Result<String, Box<dyn Error>>
where
    S: AsRef<str>,
{
    let rg = RegexSet::new(patters)?;

    let mut res = String::with_capacity(input.len());

    for word in input.split_inclusive(char::is_whitespace) {
        if rg.is_match(word) {
            append_converted_word(word, &mut res, case);
        } else {
            res.push_str(word);
        }
    }

    Ok(res)
}

fn append_converted_word(word: &str, dst: &mut String, case: Case) {
    match case {
        Case::Snake => write_word_snake(word, dst),
        Case::Camel => write_word_camel(word, dst),
    }
}

fn write_word_snake(word: &str, dst: &mut String) {
    let mut whitespace = true;

    for c in word.chars() {
        if c.is_uppercase() {
            if !whitespace {
                dst.push('_');
            }
        }

        whitespace = c.is_whitespace();

        dst.push(c.to_ascii_lowercase());
    }
}

fn write_word_camel(word: &str, dst: &mut String) {
    let mut underscore = false;

    for c in word.chars() {
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
            Case::Camel => self.append_cammel(chars, dst),
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
        let res = convert(INPUT, &["cammelCase"], Case::Snake).unwrap();
        assert_eq!(
            res,
            "word CammelCase Word cammel_case CAMMELCASE anotherOfThisKind snake_case _snake_case"
        );

        let res = convert(INPUT, &[".ammelCase"], Case::Snake).unwrap();
        assert_eq!(
            res,
            "word cammel_case Word cammel_case CAMMELCASE anotherOfThisKind snake_case _snake_case"
        );
    }

    #[test]
    fn snake_to_cammel() {
        let res = convert(INPUT, &["snake_case"], Case::Camel).unwrap();
        assert_eq!(
            res,
            "word CammelCase Word cammelCase CAMMELCASE anotherOfThisKind snakeCase SnakeCase"
        );

        let res = convert(INPUT, &[r".snake_case"], Case::Camel).unwrap();
        assert_eq!(
            res,
            "word CammelCase Word cammelCase CAMMELCASE anotherOfThisKind snake_case SnakeCase"
        );
    }
}
