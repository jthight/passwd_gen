use clap::{App, Arg};
use std::{ io};

const PASSWORD_LEN: &str = "30";
const PASSWORD_NUM: &str = "1";

#[derive(Debug)]
pub struct Args {
    pub outfile: String,
    pub overwrite: bool,
    pub silent: bool,
    pub special: bool,
    pub clipboard: bool,
    pub length: usize,
    pub number: usize,
}

#[derive(Debug)]
pub enum ArgError {
    OutputFound(String),
}

pub type ArgResult = Result<(), ArgError>;

impl Args {
    pub fn parse() -> Self {
        let matches = App::new("passwd_gen")
            .version("0.0.1")
            .author("John Hight <john@hight.net")
            .about("Generates random passwords with alphabetic and special characters")
            .arg(
                Arg::new("outfile")
                    .short('o')
                    .long("outfile")
                    .takes_value(true)
                    .help("Write output to a file instead of stdout"),
            )
            .arg(
                Arg::new("length")
                    .short('l')
                    .long("length")
                    .takes_value(true)
                    .default_value(PASSWORD_LEN)
                    .help("Length of password: default is 30 characters"),
            )
            .arg(
                Arg::new("number")
                    .short('n')
                    .long("number")
                    .takes_value(true)
                    .default_value(PASSWORD_NUM)
                    .help("Number of passwords: default is 1 password"),
            )
            .arg(
                Arg::new("silent")
                    .short('s')
                    .long("silent")
                    .help("Silent mode: no display of password on screen"),
            )
            .arg(
                Arg::new("special")
                    .short('e')
                    .long("extend")
                    .help("Enable extended special characters beyond default"),
            )
            .arg(
                Arg::new("overwrite")
                    .short('y')
                    .long("overwrite")
                    .help("Option to allow overwriting a file."),
            )
            .arg(
                Arg::new("clipboard")
                    .short('c')
                    .long("clipboard")
                    .help("Send passwords to clipboard."),
            )
            .get_matches();
        let outfile = matches.value_of("outfile").unwrap_or_default().to_string();
        let length = matches
            .value_of("length")
            .unwrap_or_default()
            .to_string()
            .parse()
            .unwrap();
        let number = matches
            .value_of("number")
            .unwrap_or_default()
            .to_string()
            .parse()
            .unwrap();
        let silent = matches.is_present("silent");
        let special = matches.is_present("special");
        let overwrite = matches.is_present("overwrite");
        let clipboard = matches.is_present("clipboard");
        Self {
            outfile,
            overwrite,
            silent,
            special,
            clipboard,
            length,
            number,
        }
    }
    pub fn validate_args(&self) -> ArgResult {
        if !self.outfile.is_empty()
            && std::path::Path::new(&self.outfile).exists()
            && &self.outfile != "/dev/null"
            && !self.overwrite
        {
            eprint!(
                "File: '{}' exists, do you want to overwrite? [Y/n]: ",
                &self.outfile
            );
            if !yes_no("y") {
                return Err(ArgError::OutputFound(format!(
                    "File '{}' not overwritten. To overwrite use -y or -overwrite option.",
                    self.outfile
                )));
            }
            eprintln!();
        }

        Ok(())
    }
}

fn yes_no(default: &str) -> bool {
    // function will return true if yes and false if no.
    if default.is_empty() {
        return false;
    }
    let default = default.to_ascii_lowercase();
    if &default[..1] != "n" && &default[..1] != "y" {
        return false;
    }
    #[allow(unused_assignments)]
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    loop {
        buffer = "".to_string();
        match stdin.read_line(&mut buffer) {
            Ok(x) => {
                if x > 5 {
                    return false;
                }
                match buffer.trim().to_ascii_lowercase().as_str() {
                    "" => return &default[..1] == "y",
                    "y" | "yes" => return true,
                    "n" | "no" => return false,
                    _ => eprintln!("Reply with \"y, yes or n, no\" in UPPERCASE or lowercase."),
                }
            }
            Err(e) => eprintln!("error: {}", e),
        }
        eprint!("Input [y/n] or [enter] for default {}: ", &default);
    }
}
