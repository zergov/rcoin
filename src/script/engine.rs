use crate::hash::sha256;
use crate::script::opcodes::*;
use crate::transactions::Transaction;

pub fn new() -> Engine {
    Engine {
        script: vec![],
        stack: vec![],
        pc: 0,
        transaction: None,
        vin: 0,
    }
}

pub struct Engine {
    // the script to execute.
    script: Vec<u8>,

    // the stack for the execution of the script.
    stack: Vec<Vec<u8>>,

    // the program counter is the index of the current instruction.
    pc: usize,

    // the transaction this script engine will run for.
    transaction: Option<Transaction>,

    // the index of the input transaction this script engine will run for.
    vin: u32,
}

impl Engine {
    pub fn execute(&mut self, script: Vec<u8>) -> Result<bool, String> {
        self.script = script;
        self.stack = vec![];
        self.pc = 0;

        let end = self.script.len();

        while self.pc < end {
            let op_code = self.script[self.pc];
            self.pc += 1;

            println!("=================================");
            let result = match op_code {
                op if op < OP_PUSHDATA1 => self.op_push_data(op as usize),
                OP_ADD => self.op_add(),
                OP_EQUAL => self.op_equal(),
                OP_SHA256 => self.op_sha256(),
                _ => Ok(()),
            };

            if let Err(error) = result {
                return Err(error);
            }
        }

        // https://learnmeabitcoin.com/technical/script
        // The script is invalid if:
        // - The final stack is empty
        // - The top element is on the stack is 0
        // - There is more than one element left on the stack at the end of execution. 1
        // - The script exits prematurely (e.g. OP_RETURN in a NULL DATA script).
        if self.stack.len() > 1 {
            return Ok(false)
        };

        match self.stack.pop() {
            None => Ok(false),
            Some(v) => Ok(v.iter().any(|&x| x != 0)),
        }
    }

    fn op_push_data(&mut self, size: usize) -> Result<(), String> {
        let data = &self.script[(self.pc)..(self.pc + size)];

        self.stack.push(data.to_vec());
        self.pc += size;
        Ok(())
    }

    fn op_add(&mut self) -> Result<(), String> {
        let a = self.pop_i32();
        let b = self.pop_i32();

        match (a, b) {
            (Some(a), Some(b)) => {
                let result = a + b;
                self.stack.push(smallest_i32_bytes(result));
                Ok(())
            }
            _ => Err(String::from("OP_ADD: missing values on stack."))
        }

    }

    fn op_equal(&mut self) -> Result<(), String> {
        let a = self.stack.pop();
        let b = self.stack.pop();

        match (a, b) {
            (Some(a), Some(b)) => {
                println!("OP_EQUAL");
                println!("a: {:x?}", a);
                println!("b: {:x?}", b);

                let result: Vec<u8> = if a == b { vec![1] } else { vec![0] };
                self.stack.push(result);
                Ok(())
            }
            _ => Err(String::from("OP_EQUAL: missing values on stack."))
        }
    }

    fn op_sha256(&mut self) -> Result<(), String> {
        match self.stack.pop() {
            Some(data) => {
                let data = hex::encode(data);
                self.stack.push(sha256(&data.as_bytes()));
                Ok(())
            },
            None => Err(String::from("OP_SHA256: missing value on stack."))
        }
    }

    fn pop_i32(&mut self) -> Option<i32> {
        let mut a = self.stack.pop()?;

        // the bytes might not have a length of 4, so we reverse the byte order, and pad the vector
        // to a 4 bytes length. That byte array can then be parsed as a little endian i32.
        a.reverse();
        a.resize(4, 0);

        Some(i32::from_le_bytes(a.try_into().unwrap()))
    }
}

fn smallest_i32_bytes(n: i32) -> Vec<u8> {
    let [a, b, c, d]: [u8; 4] = n.to_be_bytes();

    if (n as u32 & 0xffffff00) == 0 {
        return vec![d]
    };

    if (n as u32 & 0xffff0000) == 0 {
        return vec![c, d]
    };

    if (n as u32 & 0xff000000) == 0 {
        return vec![b, c, d]
    };

    vec![a, b, c, d]
}

#[cfg(test)]
mod test {
    use super::*;
    // using some of the scripts found at https://learnmeabitcoin.com/technical/script

    #[test]
    fn test_math_puzzle_success() {
        // 4 4 ADD 8 EQUAL
        let script = hex::decode("0104010493010887").unwrap();
        assert_eq!(Ok(true), new().execute(script))
    }

    #[test]
    fn test_math_puzzle_failure() {
        // 4 1 ADD 8 EQUAL
        let script = hex::decode("0104010193010887").unwrap();
        assert_eq!(Ok(false), new().execute(script))
    }

    #[test]
    fn test_math_puzzle_add_missing_stack_value() {
        // 4 ADD 8 EQUAL
        let script = hex::decode("010493010887").unwrap();
        assert_eq!(Err(String::from("OP_ADD: missing values on stack.")), new().execute(script))
    }

    #[test]
    fn test_math_puzzle_equal_missing_stack_value() {
        // 4 4 ADD EQUAL
        let script = hex::decode("010401049387").unwrap();
        assert_eq!(Err(String::from("OP_EQUAL: missing values on stack.")), new().execute(script))
    }

    #[test]
    fn test_hash_puzzle_success() {
        // 72636f696e OP_SHA256 e49dc62d36294343898b5a0b29335600c1106b70a2827371fe1321013d764a85 OP_EQUAL
        let script = hex::decode("0572636f696ea820e49dc62d36294343898b5a0b29335600c1106b70a2827371fe1321013d764a8587").unwrap();
        assert_eq!(Ok(true), new().execute(script));
    }

    #[test]
    fn test_hash_puzzle_failure() {
        // 0000000000 OP_SHA256 e49dc62d36294343898b5a0b29335600c1106b70a2827371fe1321013d764a85 OP_EQUAL
        let script = hex::decode("050000000000a820e49dc62d36294343898b5a0b29335600c1106b70a2827371fe1321013d764a8587").unwrap();
        assert_eq!(Ok(false), new().execute(script))
    }

    #[test]
    fn test_sha256_missing_stack_value() {
        // OP_SHA256 e49dc62d36294343898b5a0b29335600c1106b70a2827371fe1321013d764a85 OP_EQUAL
        let script = hex::decode("a820e49dc62d36294343898b5a0b29335600c1106b70a2827371fe1321013d764a8587").unwrap();
        assert_eq!(Err(String::from("OP_SHA256: missing value on stack.")), new().execute(script))
    }

    // #[test]
    // fn test_p2pk_success() {
       // let keychain = crate::keys::generate_keychain();
       // let public_key = keychain.public_key_hex();
       // let script = String::from("{} OP_CHECKSIG");
    // }
}
