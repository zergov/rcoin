use openssl::nid::Nid;
use openssl::bn::{BigNum, BigNumContext};
use openssl::ec::{EcGroup, EcKey, PointConversionForm};
use openssl::hash::{hash, MessageDigest};

fn main() {
    let mut ctx = BigNumContext::new().unwrap();
    let curve_nid = Nid::SECP256K1;
    let group = EcGroup::from_curve_name(curve_nid).unwrap();

    let key = EcKey::generate(&group).unwrap();
    let private_key = key.private_key();
    let public_key = key.public_key();

    let mut pub_x = BigNum::new().unwrap();
    let mut pub_y = BigNum::new().unwrap();
    public_key
        .affine_coordinates(&group.as_ref(), &mut pub_x, &mut pub_y, &mut ctx)
        .unwrap();

    let public_key_bytes = public_key
        .to_bytes(&group, PointConversionForm::UNCOMPRESSED, &mut ctx)
        .unwrap();

    let private_key_str = private_key.to_hex_str().unwrap();
    let public_key_str = format!("04{}{}", pub_x.to_hex_str().unwrap(), pub_y.to_hex_str().unwrap());

    println!("Rcoin address: {}", generate_address(&public_key_bytes));
    println!("==================================");
    println!("private key:\t{}", private_key_str);
    println!("public key:\t{}", public_key_str);
    println!("x: {}", pub_x.to_hex_str().unwrap());
    println!("y: {}", pub_y.to_hex_str().unwrap());
}

// Public key to Rcoin address.
//
// Addresses generation follows the same rules as Bitcoin:
// https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch04.asciidoc#bitcoin-addresses
fn generate_address(public_key: &[u8]) -> String {
    let payload = hash(MessageDigest::sha256(), public_key).unwrap();
    let payload = hash(MessageDigest::ripemd160(), &payload).unwrap();
    let mut payload = payload.to_vec();

    // Base58Check prefix version: 0x00 for addresses.
    payload.insert(0, 0x00);

    let checksum = hash(MessageDigest::sha256(), &payload).unwrap();
    let checksum = hash(MessageDigest::sha256(), &checksum).unwrap();
    let mut checksum = checksum
        .to_vec()
        .into_iter()
        .take(4)
        .collect();

    payload.append(&mut checksum);

    bs58::encode(payload).into_string()
}
