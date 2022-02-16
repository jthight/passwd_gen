use rand::Rng;

/// Standard set of Alphanumeric and Special characters
/// used to generate a password with default settings.
const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789*&^%$#@!";
/// Extended set of Special characters used to generate
/// a password when the -e or --extend option is set.
const ESPCHARSET: &[u8] = b"~`()_-+={[}]|\\:;\"'<,>.?/";

/// Returns a one or a number of passwords
/// with the same length as a line separated string.
pub fn collect_passwords(length: usize, number: usize, extend: bool) -> String {
    let mut buffer = String::new();

    for n in 0..number {
        if n > 0 {
            buffer.push_str("\r\n");
        }
        buffer.push_str(&create_password(length, extend));
    }
    buffer
}

/// Generates a single password of specified length
/// and extended set of special characters if requested.
pub fn create_password(length: usize, extend: bool) -> String {
    let mut rng = rand::thread_rng();
    let mut pwcharset: Vec<u8> = Vec::new();
    let mut first_char: bool;

    pwcharset.extend(CHARSET);
    if extend {
        pwcharset.extend(ESPCHARSET);
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
    password
}