//! # passwd_gen
//! A Cli utility to generate random passwords of
//! a user defined length and quantity.
//! My inspiration for this utility came from the
//! "Rust Cookbook" in [1.1. Generate Random Values](https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html#create-random-passwords-from-a-set-of-user-defined-characters)
//!
//! ## Help on Usage
//! ```text
//! $ passwd_gen -h
//! passwd_gen 0.1.0
//! John Hight <https://github.com/jthight>
//! Generates random passwords with alphanumeric and special characters
//!
//! USAGE:
//! passwd_gen [OPTIONS]
//!
//! OPTIONS:
//! -c, --clipboard            Send passwords to clipboard.
//! -e, --extend               Enable extended special characters beyond default
//! -h, --help                 Print help information
//! -l, --length <length>      Length of password: default is 30 characters [default: 30]
//! -n, --number <number>      Number of passwords: default is 1 password [default: 1]
//! -o, --outfile <outfile>    Write output to a file instead of stdout
//! -s, --silent               Silent mode: no display of password on screen
//! -V, --version              Print version information
//! -y, --overwrite            Option to allow overwriting a file.
//! ```
//!
//! ## Example Usage
//! ```text
//! $ passwd_gen
//! f#lVvpIMdWrHG8K2ACr!wHYbIQm9JL
//!
//! $ passwd_gen -e
//! f1!kd7:`[+^MwwABT#G|""OT!")cHI
//!
//! $ passwd_gen -e -l 60
//! JQ76M6*e8Z=|}uh|w]i-F\~<G("06kWPS9Sb.vJ<3~Bik,r4MxHL#75fWB%t
//!
//! $ passwd_gen -e -l 60 -n 10
//! 9OdN/-7?""_iz5a^V=_GcPS;go|Zb&m13SY;Gj!VdCgd:$d-h"(w>h[e^mcB
//! 7HO+oM5U(`.GEE4:JS#%w1E}dQFfU`&BQ-Rnkr`/xVfai'_iPM.#G8dNcI{)
//! N3SVdyb]+~qRK]xS5b$'|t)9bnVkY`6o2FpoU*>ll[z{,"Q1b?V7J!#Q!gAz
//! WP1^4<dW[J$iAF\r!6[V0`F(M`kWQ>P^?};V6,IA6O{\F|:-v@(]A]8=]U{y
//! 5*W'dbjP;H7V9Tj=e?cPC5x!q'=<!}~>.~K"cTMB!5!duN_XP#Cjq!(%IwxM
//! D0H$?5S=rnAv`P:NqBCjS<uaR';na=Dkg8C;_35xpc}r;m\-]rS9vMwS{n\A
//! s21tt6$y`k7GmTLLuB++uMb:XrDXO?13.?MB~*Q3#1ZAChq8)j*BO<?\]p{~
//! U$kr6lDRE;Ug9e1/@[ff)ug=d,|k#920zOkpvLKq].;blt9{HEvQH'>wH0_v
//! 5~u(."]1d1RB_W`@IayFbJdUg(P(Co2vAB.qiJ4l@h1$6P6dTPdx.F^M;Vrn
//! &{ojLxK'gdoqM"nTURqVskh,jr,?\'1L4O/SH[_f[RoJ2uc=2M+qf1iYpTS,
//!
//! ```
//!
//! ## Example: save to file: password.txt
//! ```text
//! $ passwd_gen -e -l 60 -n 10 -o password.txt
//! Passwords written to: password.txt
//! B1VJk8>ZfaO[YjhlZ*=J`+.C<<Q6%6C-7^QlTWCqz5@E4h41:_edN>h;KOjB
//! nzG)Q!\_Y/`1?h<NB@bJ6zi:&cUf`Pb;`dN!6"suTOLn)oC}ZOb,4@/d?8mw
//! 0^MN7h4*"A=Nec)j[Kv(IW*)}R!RS62n\s$kj1M>p7"yBDFmXe&/@:&)vn8X
//! 3IJ$kiQF&i/t3'\[`nFdRrAB+j7KK"zgM\xSmSI~}y!&Uc4u^3`dAx=%?>6'
//! cvp&%Dq3I!8OGX?t}3FP2_W|cQ<.Sat3$Mm>_#Sdejnt#*9c!gj7kn=#=?S(
//! xMy9aG}s0vHA|,D'^q*}QMM@\kZB+J(S}hI`1|s/8PeFi}G6wI>Da:'@D[U+
//! #vygcPz1$t,GI,[>sf99{ejoO9'v&xp%8XlCK7].]M2zhTwX`Lr}`48nNX~8
//! w~gCw!D:tzj](jp?b5EhkYPSNEA_g<p/N2O{WMxXc23b207;~<S^3|U^#+n7
//! bWm50k"LAFoxzvtp)f5>$becSq('I*y|XwjMwIyIU=wOP^4uYoAH|cb%P\wz
//! ~!1=?[bwiSSiebjMYK-&;-r0Hb#jO^L>7@OR1G|MCVioJzA{/CU3rN|V"OG8
//! ```
//!
//! ## Example: save to clipboard
//! ```text
//! $ passwd_gen -c
//! Passwords written to: clipboard
//! dCt%HUi0FvW#TCu!IjOP@ej5sEzyKk
//! ```
//! Note: all options can be used in any combination.
//!
//! ## Characters used to generate passwords
//! ``` rust
//! // Standard set of Alphanumeric and Special characters
//! // used to generate a password with default settings.
//! const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
//!                         abcdefghijklmnopqrstuvwxyz\
//!                         0123456789*&^%$#@!";
//! // Extended set of Special characters used to generate
//! // a password when the -e or --extend option is set.
//! const ESPCHARSET: &[u8] = b"~`()_-+={[}]|\\:;\"'<,>.?/";
//!
//! ```
//!

mod args;
mod password;

use crate::args::Args;
use crate::password::collect_passwords;
use arboard::Clipboard;
use std::fs::File;
use std::io;
use std::io::prelude::*;

/// This utility will return system generated errors
/// and a internal error if a file will be overwritten without user input.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match Args::validate_args(&args) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{:?}", e);
            return Ok(());
        }
    }

    // Generate the passwords
    let buffer = collect_passwords(args.length, args.number, args.extend);

    // Write passwords to file is requested
    if !args.outfile.is_empty() {
        let mut file = File::create(&args.outfile)?;
        file.write_all(buffer.as_bytes())?;
        if !args.silent {
            eprintln!("Passwords written to: {}", args.outfile);
        }
    }

    // Write passwords to clipboard is requested
    if args.clipboard {
        let mut ctx: Clipboard = Clipboard::new()?;
        ctx.set_text(buffer.clone())?;
        if !args.silent {
            eprintln!("Passwords written to: clipboard");
        }
    }

    // Write passwords to output stream is not silent
    if !args.silent {
        write!(&mut io::stdout(), "{}", buffer)?;
    }

    Ok(())
}
