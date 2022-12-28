use openssl::nid::Nid;
use openssl::bn::{BigNum, BigNumContext};
use openssl::ec::{EcGroup, EcKey, PointConversionForm};

pub struct Keychain {
    key_pair: EcKey<openssl::pkey::Private>,
}

pub fn generate_keychain() -> Keychain {
    let curve = EcGroup::from_curve_name(Nid::SECP256K1).unwrap();
    let key_pair = EcKey::generate(&curve).unwrap();

    Keychain { key_pair }
}

impl Keychain {
    pub fn private_key_hex(&self) -> String {
        self.key_pair.private_key().to_hex_str().unwrap().to_string()
    }

    pub fn private_key_pem(&self) -> Vec<u8>{
        self.key_pair.private_key_to_pem().unwrap()
    }

    pub fn public_key_bytes(&self) -> Vec<u8> {
        let mut ctx = BigNumContext::new().unwrap();
        let public_key = self.key_pair.public_key();
        let group = self.key_pair.group();

        public_key
            .to_bytes(group, PointConversionForm::UNCOMPRESSED, &mut ctx)
            .unwrap()
    }

    pub fn public_key_hex(&self) -> String {
        let mut ctx = BigNumContext::new().unwrap();
        let mut x = BigNum::new().unwrap();
        let mut y = BigNum::new().unwrap();
        let public_key = self.key_pair.public_key();
        let group = self.key_pair.group();

        public_key
            .affine_coordinates(group, &mut x, &mut y, &mut ctx)
            .unwrap();

        format!("04{}{}", x.to_hex_str().unwrap(), y.to_hex_str().unwrap())
    }

    pub fn public_key_pem(&self) -> Vec<u8> {
        self.key_pair.public_key_to_pem().unwrap()
    }
}
