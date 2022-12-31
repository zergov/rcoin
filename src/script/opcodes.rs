// https://en.bitcoin.it/wiki/Script

// constants
/// The next byte contains the number of bytes to be pushed onto the stack.
pub const OP_PUSHDATA1: u8 = 0x4c;

// Bitwise logic
/// Returns 1 if the inputs are exactly equal, 0 otherwise.
pub const OP_EQUAL: u8 = 0x87;

// Arithmetic
//
// Note: Arithmetic inputs are limited to signed 32-bit integers, but may overflow their output.
// If any input value for any of these commands is longer than 4 bytes, the script must abort and
// fail. If any opcode marked as disabled is present in a script - it must also abort and fail.
/// a is added to b
pub const OP_ADD: u8 = 0x93;

// Crypto
/// The input is hashed using SHA-256.
pub const OP_SHA256: u8 = 0xa8;
