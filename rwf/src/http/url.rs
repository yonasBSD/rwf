//! URL handling helpers.

/// Decode a string encoded with URL encoding.
///
/// # Arguments
///
/// * `s` - The string to decode.
///
/// # Example
///
/// ```
/// use rwf::http::urldecode;
///
/// let url = "?foo=bar&hello=world%20";
/// let decoded = urldecode(url);
/// assert_eq!(decoded, "?foo=bar&hello=world ");
/// ```
///
pub fn urldecode(s: &str) -> String {
    let mut result = String::new();
    let mut iter = s.chars().peekable();

    while let Some(c) = iter.next() {
        match c {
            '%' => {
                let mut num = String::new();

                loop {
                    match iter.peek() {
                        Some(&c) if c.is_ascii_hexdigit() => {
                            num.push(iter.next().unwrap());
                        }

                        _ => {
                            if let Ok(byte) = u8::from_str_radix(&num, 16) {
                                result.push(byte as char);
                            }

                            break;
                        }
                    }
                }
            }

            '+' => result.push(' '),

            c => result.push(c),
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_urldecode() {
        let url = "?foo=bar&hello=world";
        let decoded = urldecode(url);
        assert_eq!(decoded, "?foo=bar&hello=world");

        let url = "?foo=bar&hello=world%20&apples%3Doranges";
        let decoded = urldecode(url);
        assert_eq!(decoded, "?foo=bar&hello=world &apples=oranges");
    }
}
