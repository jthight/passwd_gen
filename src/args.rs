use clap::{App, Arg};
use std::io;

/// The default length of passwords generated.
const PASSWORD_LEN: &str = "30";
/// The default number of passwords generated.
const PASSWORD_NUM: &str = "1";
/// Arguments passed by the command line string
#[derive(Debug)]
pub struct Args {
    pub outfile: String,
    pub overwrite: bool,
    pub silent: bool,
    pub extend: bool,
    pub clipboard: bool,
    pub length: usize,
    pub number: usize,
}

/// Errors returned by ArgResult
#[derive(Debug)]
pub enum ArgError {
    OutputFound(String),
}

/// Return results for the function "validate_args" used to validate the command line arguments.
pub type ArgResult = Result<(), ArgError>;

impl Args {
    pub fn parse() -> Self {
        let matches = App::new("passwd_gen")
            .version("0.1.0")
            .author("John Hight <https://github.com/jthight>")
            .about("Generates random passwords with alphanumeric and special characters")
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
                Arg::new("extend")
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
        let extend = matches.is_present("extend");
        let overwrite = matches.is_present("overwrite");
        let clipboard = matches.is_present("clipboard");
        Self {
            outfile,
            overwrite,
            silent,
            extend,
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
        }

        Ok(())
    }
}

/// Evaluates the response to a yes/no prompt.
/// This function will return true for a "yes'
/// response and false for a "no" response.
/// The variable "default" will be the response
/// returned if the response is an \[enter\].
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
