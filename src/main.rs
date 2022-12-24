use openssl::nid::Nid;
use openssl::bn::{BigNum, BigNumContext};
use openssl::ec::{EcGroup, EcKey};

fn main() {
    let mut ctx = BigNumContext::new().unwrap();
    let curve_nid = Nid::SECP256K1;
    let group = EcGroup::from_curve_name(curve_nid).unwrap();
    let key = EcKey::generate(&group).unwrap();

    let private_key = key.private_key();
    let public_key = &key.public_key();
    let mut pub_x = BigNum::new().unwrap();
    let mut pub_y = BigNum::new().unwrap();
    public_key
        .affine_coordinates(&group.as_ref(), &mut pub_x, &mut pub_y, &mut ctx)
        .unwrap();

    let private_key_str = private_key.to_hex_str().unwrap();
    let public_key_str = format!("04{}{}", pub_x.to_hex_str().unwrap(), pub_y.to_hex_str().unwrap());

    println!("private key ====================");
    println!("k:\t{}", private_key_str);
    println!("");
    println!("public key ====================");
    println!("x:\t{}", pub_x.to_hex_str().unwrap());
    println!("y:\t{}", pub_y.to_hex_str().unwrap());
    println!("k:\t{}", public_key_str);
}
