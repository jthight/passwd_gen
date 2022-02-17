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
/// Generates a single password of specified length
/// and extended set of special characters if requested.
///
/// ```rust
/// use passwd_gen::password::collect_passwords;
///
/// // Collect a single password with Alphanumeric and *&^%$#@! characters.
/// let password: String = collect_passwords(30, 1, false);
/// assert!(password.is_ascii());
/// assert_eq!(password.len(), 30);
///
/// // Collect a single password with Alphanumeric
/// // and *&^%$#@!~`()_-+={[}]|\:;"'<,>.?/ characters.
/// let password: String = collect_passwords(30, 1, true);
/// assert!(password.is_ascii());
/// assert_eq!(password.len(), 30);
///
/// // Collect two passwords with the same length into the a
/// // string with two lines separated by (cr/lf).
/// let passwords: String = collect_passwords(30, 2, false);
/// assert!(passwords.is_ascii());
/// assert_eq!(passwords.len(), 62);
/// assert!(!passwords.ends_with("\r\n"));
/// let vp: Vec<&str> = passwords.split("\r\n").collect();
/// assert_eq!(vp[0].len(), 30);
/// assert!(vp[0].is_ascii());
/// assert_eq!(vp[1].len(), 30);
/// assert!(vp[1].is_ascii());
///
/// // Collect ten passwords is 10 lines of 40 characters with no (cr/lf) at the end.
/// let passwords: String = collect_passwords(40, 10, false);
/// assert!(passwords.is_ascii());
/// assert_eq!(passwords.len(), 418);
/// assert!(passwords.ends_with("\r\n"));
/// ```
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
///
/// ```rust
/// use passwd_gen::password::create_password;
///
/// // Create a password with Alphanumeric and *&^%$#@! characters.
/// let password: String = create_password(30, false);
/// assert!(password.is_ascii());
/// assert_eq!(password.len(), 30);
///
/// // Create a password with Alphanumeric
/// // and *&^%$#@!~`()_-+={[}]|\:;"'<,>.?/ characters.
/// let password: String = create_password(30, true);
/// assert!(password.is_ascii());
/// assert_eq!(password.len(), 30);
/// ```
/// Note: the first character of the password will not be one of the extended special characters.
/// Also, as a footnote the base itorator for this function was taken from the
/// "Rust Cookbook" in [1.1. Generate Random Values](https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html#create-random-passwords-from-a-set-of-user-defined-characters)
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn password_test() {
        assert_eq!(2 + 2, 4);
        // Create a password with Alphanumeric and *&^%$#@! characters.
        let password: String = create_password(30, false);
        assert!(password.is_ascii());
        assert_eq!(password.len(), 30);
        assert!(!password.ends_with("\r\n"));

        // Create a password with Alphanumeric
        // and *&^%$#@!~`()_-+={[}]|\\:;\"'<,>.?/ characters.
        let password: String = create_password(30, true);
        assert!(password.is_ascii());
        assert_eq!(password.len(), 30);
        assert!(!password.ends_with("\r\n"));
    }
    #[test]
    fn passwords_test() {
        // Collect a single password with Alphanumeric and *&^%$#@! characters.
        let password: String = collect_passwords(30, 1, false);
        assert!(password.is_ascii());
        assert_eq!(password.len(), 30);
        assert!(!password.ends_with("\r\n"));

        // Collect a single password with Alphanumeric
        // and *&^%$#@!~`()_-+={[}]|\\:;\"'<,>.?/ characters.
        let password: String = collect_passwords(30, 1, true);
        assert!(password.is_ascii());
        assert_eq!(password.len(), 30);
        assert!(!password.ends_with("\r\n"));

        // Collect two passwords with the same length into the a string with two lines.
        let passwords: String = collect_passwords(30, 2, false);
        assert!(passwords.is_ascii());
        assert_eq!(passwords.len(), 62);
        assert!(!passwords.ends_with("\r\n"));
        let vp: Vec<&str> = passwords.split("\r\n").collect();
        assert_eq!(vp[0].len(), 30);
        assert!(vp[0].is_ascii());
        assert_eq!(vp[1].len(), 30);
        assert!(vp[1].is_ascii());

        // Collect ten passwords is 10 lines of 40 characters with no (cr/lf) at the end.
        let passwords: String = collect_passwords(40, 10, false);
        assert!(passwords.is_ascii());
        assert_eq!(passwords.len(), 418);
        assert!(!passwords.ends_with("\r\n"));

    }
}
