use openssl::hash::{hash, MessageDigest};

fn execute(script: String) -> Result<bool, String> {
    let mut stack: Vec<String> = Vec::new();
    for token in script.split_whitespace() {
        match token {
            "ADD" => {
                match (stack.pop(), stack.pop()) {
                    (Some(a), Some(b)) => {
                        let a: u32 = a.parse().unwrap();
                        let b: u32 = b.parse().unwrap();
                        stack.push((a + b).to_string());
                    },
                    _ => return Err(String::from("ADD: missing values on stack.")),
                }
            },
            "EQUAL" => {
                match (stack.pop(), stack.pop()) {
                    (Some(a), Some(b)) => {
                        let out = if a == b { "1" } else { "0" };
                        stack.push(String::from(out))
                    },
                    _ => return Err(String::from("EQUAL: missing values on stack.")),
                }
            },
            "SHA256" => {
                match stack.pop() {
                    Some(value) => {
                        let sha256 = hash(MessageDigest::sha256(), value.as_bytes()).unwrap();
                        stack.push(hex::encode(sha256));
                    }
                    _ => return Err(String::from("SHA256: missing value on stack."))
                }
            }
            _ => stack.push(token.to_string()),
        };
    }

    if stack.len() > 1 {
        return Ok(false);
    };

    match stack.pop() {
        None => Ok(false),
        Some(s) => Ok(s != "0"),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // using some of the scripts found at https://learnmeabitcoin.com/technical/script

    #[test]
    fn test_math_puzzle_success() {
        let script = String::from("4 4 ADD 8 EQUAL");
        assert_eq!(Ok(true), execute(script))
    }

    #[test]
    fn test_math_puzzle_failure() {
        let script = String::from("1 4 ADD 8 EQUAL");
        assert_eq!(Ok(false), execute(script))
    }

    #[test]
    fn test_math_puzzle_add_missing_stack_value() {
        let script = String::from("4 ADD 8 EQUAL");
        assert_eq!(Err(String::from("ADD: missing values on stack.")), execute(script))
    }

    #[test]
    fn test_math_puzzle_equal_missing_stack_value() {
        let script = String::from("4 4 ADD EQUAL");
        assert_eq!(Err(String::from("EQUAL: missing values on stack.")), execute(script))
    }

    #[test]
    fn test_hash_puzzle_success() {
        let script = String::from("rcoin SHA256 660e4502ce8f393eb5d5710febc339a58778bce175e4647ce50f8639786d132a EQUAL");
        assert_eq!(Ok(true), execute(script))
    }

    #[test]
    fn test_hash_puzzle_failure() {
        let script = String::from("something_else SHA256 660e4502ce8f393eb5d5710febc339a58778bce175e4647ce50f8639786d132a EQUAL");
        assert_eq!(Ok(false), execute(script))
    }

    #[test]
    fn test_sha256_missing_stack_value() {
        let script = String::from("SHA256 660e4502ce8f393eb5d5710febc339a58778bce175e4647ce50f8639786d132a EQUAL");
        assert_eq!(Err(String::from("SHA256: missing value on stack.")), execute(script))
    }

    #[test]
    fn test_p2pk_success() {
       let keychain = crate::keys::generate_keychain();
       let public_key = keychain.public_key_hex();
       let script = String::from("{} OP_CHECKSIG");
    }
}
