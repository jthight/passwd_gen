use std::fs::File;
use std::io::prelude::*;
use arboard::Clipboard;
use rand::Rng;
use passwd_gen::{args::Args};
use std::fmt::Write as FmtWrite;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();
    match Args::validate_args(&args) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{:?}", e);
            return Ok(());
        }
    }

    #[allow(unused_variables)]
        let Args {
        outfile,
        overwrite,
        silent,
        special,
        clipboard,
        length,
        number,
    } = args;

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789*&^%$#@!";
    const ESPCHARSET: &[u8] = b"~`()_-+={[}]|\\:;\"'<,>.?/";
    let mut rng = rand::thread_rng();

    let mut pwcharset: Vec<u8> = Vec::new();
    // #[allow(unused_assignments)]
    let mut first_char: bool;
    let mut buffer = String::new();
    pwcharset.extend(CHARSET);
    if special {
        pwcharset.extend(ESPCHARSET);
    }

    for _n in 0..number {
        if pwcharset.len() > CHARSET.len() {
            first_char = true;
        } else {
            first_char = false;
        }
        let password: String = (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..pwcharset.len());
                if first_char && idx > CHARSET.len() {
                    first_char = false;
                    pwcharset[idx - ESPCHARSET.len()] as char
                } else {
                    pwcharset[idx] as char
                }
            })
            .collect();
        // buffer.push(password);
        writeln!(&mut buffer, "{}", password)?;
    }


    if !silent {
        println!("{}", buffer);
    }
    if !outfile.is_empty() {
        let mut file = File::create(&outfile)?;
        file.write_all(buffer.as_bytes())?;
        if !silent {
            println!("Passwords written to: {}", outfile);
        }
    }
    if clipboard {
        let mut ctx: Clipboard = Clipboard::new()?;
        ctx.set_text(buffer.to_string())?;
        if !silent {
            println!("Passwords written to: clipboard");
        }
    }
    Ok(())
}
