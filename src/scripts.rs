struct ScriptEngine {
    script: Vec<u8>,
    stack: Vec<Vec<u8>>,
    ip: u32,
}

impl ScriptEngine {
    pub fn new() -> ScriptEngine {
        ScriptEngine {
            script: vec![],
            stack: vec![],
            ip: 0,
        }
    }

    pub fn execute(&mut self, script: Vec<u8>) -> Result<bool, String> {
        Ok(true)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // using some of the scripts found at https://learnmeabitcoin.com/technical/script

    #[test]
    fn test_math_puzzle_success() {
        // 4 4 ADD 8 EQUAL
        let script = hex::decode("0104010493010887").unwrap();
        let mut script_engine = ScriptEngine::new();
        assert_eq!(Ok(true), script_engine.execute(script))
    }

    #[test]
    fn test_math_puzzle_failure() {
        // 4 1 ADD 8 EQUAL
        let script = hex::decode("0104010193010887").unwrap();
        let mut script_engine = ScriptEngine::new();
        assert_eq!(Ok(false), script_engine.execute(script))
    }

    // #[test]
    // fn test_math_puzzle_add_missing_stack_value() {
        // let script = String::from("4 ADD 8 EQUAL");
        // assert_eq!(Err(String::from("ADD: missing values on stack.")), execute(script))
    // }

    // #[test]
    // fn test_math_puzzle_equal_missing_stack_value() {
        // let script = String::from("4 4 ADD EQUAL");
        // assert_eq!(Err(String::from("EQUAL: missing values on stack.")), execute(script))
    // }

    // #[test]
    // fn test_hash_puzzle_success() {
        // let script = String::from("rcoin SHA256 660e4502ce8f393eb5d5710febc339a58778bce175e4647ce50f8639786d132a EQUAL");
        // assert_eq!(Ok(true), execute(script))
    // }

    // #[test]
    // fn test_hash_puzzle_failure() {
        // let script = String::from("something_else SHA256 660e4502ce8f393eb5d5710febc339a58778bce175e4647ce50f8639786d132a EQUAL");
        // assert_eq!(Ok(false), execute(script))
    // }

    // #[test]
    // fn test_sha256_missing_stack_value() {
        // let script = String::from("SHA256 660e4502ce8f393eb5d5710febc339a58778bce175e4647ce50f8639786d132a EQUAL");
        // assert_eq!(Err(String::from("SHA256: missing value on stack.")), execute(script))
    // }

    // #[test]
    // fn test_p2pk_success() {
       // let keychain = crate::keys::generate_keychain();
       // let public_key = keychain.public_key_hex();
       // let script = String::from("{} OP_CHECKSIG");
    // }
}
