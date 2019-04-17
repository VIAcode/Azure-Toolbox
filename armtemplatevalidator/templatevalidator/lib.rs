use colored::*;
use failure::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone)]
pub struct Files {
    pub files: Vec<PathBuf>,
}

impl Files {
    pub fn new(input_path: Vec<String>) -> Files {
        let mut files = Vec::new();
        for input in input_path {
            let path = PathBuf::from_str(&input).unwrap();
            let mut file = match Self::get_all_files(&path) {
                Ok(file) => file,
                Err(e) => {
                    println!("Error reading files: {}", e);
                    std::process::exit(1);
                }
            };
            files.append(&mut file);
        }
        Files { files }
    }

    fn get_all_files(path: &PathBuf) -> Result<Vec<PathBuf>, Error> {
        let mut result = Vec::new();
        if path.is_dir() {
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let file_type = entry.file_type()?;
                if file_type.is_dir() {
                    result.append(&mut Self::get_all_files(&entry.path())?);
                } else {
                    result.push(entry.path());
                }
            }
        } else {
            result.push(path.clone());
        }
        Ok(result)
    }
}

pub struct Templates {
    pub templates: Vec<PathBuf>,
}

impl Templates {
    pub fn new(files: &Files) -> Templates {
        let templates = files
            .files
            .to_owned()
            .into_iter()
            .filter(|path| path.clone().to_string_lossy().ends_with(".json"))
            .collect();
        Templates { templates }
    }

    pub fn validate(&self) {
        let mut are_mistakes_present = false;
        for template in self.templates.clone() {
            let mut f = File::open(&template).unwrap();
            let mut contents = String::new();
            f.read_to_string(&mut contents).unwrap();

            let json_errors = Self::check_json(&contents);
            if !json_errors.is_empty() {
                are_mistakes_present = true;
                for error in json_errors {
                    match os_type::current_platform().os_type {
                        os_type::OSType::Unknown => {
                            let message =
                                format!("The JSON {:?} is Invalid! Error: {}", &template, error);
                            println!("{}", message);
                        }
                        _ => {
                            let message =
                                format!("The JSON {:?} is Invalid! Error: {}", &template, error)
                                    .red();
                            println!("{}", message);
                        }
                    }
                }
            } else {
                let mistakes = Self::line_checker(&contents);
                if !mistakes.is_empty() {
                    are_mistakes_present = true;
                    match os_type::current_platform().os_type {
                        os_type::OSType::Unknown => {
                            let message = format!("The template {:?} is Invalid!", &template);
                            println!("{}", message);
                        }
                        _ => {
                            let message = format!("The template {:?} is Invalid!", &template).red();
                            println!("{}", message);
                        }
                    }
                    mistakes.print_me();
                }
            }
        }

        if !are_mistakes_present {
            println!("All Templates Are Valid!");
        }
    }

    pub fn check_json(s: &str) -> Vec<serde_json::Error> {
        let mut errors = Vec::new();
        if let Err(e) = serde_json::from_str::<serde_json::Value>(&s) {
            errors.push(e);
        }
        errors
    }

    fn line_checker(contents: &str) -> Mistakes {
        let mut mistakes = Vec::new();
        for (num, line) in contents.lines().enumerate() {
            if line.chars().any(|x| x.is_alphabetic()) {
                if (line.contains('"') || line.contains('\''))
                    && (!line.is_quote_balanced('\'') || !line.is_quote_balanced('"'))
                    {
                        let mistake = Mistake::new(line, num + 1);
                        mistakes.push(mistake);
                    }
                if (line.contains(']') || line.contains('}') || line.contains(')'))
                    && (!line.is_balanced('(', ')')
                    || !line.is_balanced('{', '}')
                    || !line.is_balanced('[', ']'))
                    {
                        let mistake = Mistake::new(line, num + 1);
                        mistakes.push(mistake);
                    }
            }
        }
        Mistakes { mistakes }
    }
}

#[derive(Clone)]
struct Mistake {
    line: String,
    number: usize,
}

impl Mistake {
    fn new(line: &str, number: usize) -> Mistake {
        Mistake {
            line: line.to_string(),
            number,
        }
    }
}

#[derive(Clone)]
pub struct Mistakes {
    mistakes: Vec<Mistake>,
}

impl Mistakes {
    pub fn print_me(&self) {
        for mistake in self.clone().mistakes {
            match os_type::current_platform().os_type {
                os_type::OSType::Unknown => {
                    let number = mistake.number.to_string();
                    let line = mistake.line;
                    println!("{}:{}", number, line);
                }
                _ => {
                    let number = mistake.number.to_string().magenta();
                    let line = mistake.line.green();
                    println!("{}:{}", number, line);
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.mistakes.len() == 0
    }
}

pub trait Balanced {
    fn is_balanced(&self, open_bracket: char, close_bracket: char) -> bool;
    fn is_quote_balanced(&self, quote: char) -> bool;
}

impl Balanced for &str {
    fn is_balanced(&self, open_bracket: char, close_bracket: char) -> bool {
        let mut count = 0;
        let mut line = *self;
        if line.trim_end().ends_with(open_bracket) {
            line = line.trim_end();
            let len = line.len();
            line = &line[..len - 1];
        }

        for symbol in line.chars() {
            let change = match symbol {
                c if c.eq(&open_bracket) => 1,
                c if c.eq(&close_bracket) => -1,
                _ => 0,
            };

            count += change;

            if count < 0 {
                return false;
            }
        }

        count == 0
    }

    fn is_quote_balanced(&self, quote: char) -> bool {
        let count = self.chars().filter(|x| x.eq(&quote)).count();

        count % 2 == 0
    }
}
