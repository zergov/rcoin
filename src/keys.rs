use openssl::bn::{BigNum, BigNumContext};
use openssl::ec::{EcGroup, EcKey, EcPoint, PointConversionForm};
use openssl::ecdsa::EcdsaSig;
use openssl::nid::Nid;

pub struct Keychain {
    key_pair: EcKey<openssl::pkey::Private>,
}

pub fn generate_keychain() -> Keychain {
    let curve = EcGroup::from_curve_name(Nid::SECP256K1).unwrap();
    let key_pair = EcKey::generate(&curve).unwrap();

    Keychain { key_pair }
}

impl Keychain {
    pub fn from_pem(private_key_pem: &[u8]) -> Keychain {
        let key_pair = EcKey::private_key_from_pem(private_key_pem).unwrap();
        Keychain { key_pair }
    }

    pub fn private_key_hex(&self) -> String {
        self.key_pair.private_key().to_hex_str().unwrap().to_string()
    }

    pub fn private_key_pem(&self) -> Vec<u8>{
        self.key_pair.private_key_to_pem().unwrap()
    }

    pub fn public_key_uncompressed(&self) -> Vec<u8> {
        let mut ctx = BigNumContext::new().unwrap();
        let public_key = self.key_pair.public_key();
        let group = self.key_pair.group();

        public_key
            .to_bytes(group, PointConversionForm::UNCOMPRESSED, &mut ctx)
            .unwrap()
    }

    pub fn public_key_hex(&self) -> String {
        hex::encode(self.public_key_uncompressed())
    }

    pub fn public_key_pem(&self) -> Vec<u8> {
        self.key_pair.public_key_to_pem().unwrap()
    }

    pub fn sign(&self, data: &[u8]) -> Vec<u8>{
        let ecdsa_sig = EcdsaSig::sign(data, &self.key_pair).unwrap();
        ecdsa_sig.to_der().unwrap()
    }
}

pub fn verify_signature(data: &[u8], signature: &[u8], public_key: &String) -> bool {
    let mut ctx = BigNumContext::new().unwrap();
    let curve = EcGroup::from_curve_name(Nid::SECP256K1).unwrap();

    let public_key_bytes = hex::decode(public_key).unwrap();
    let public_key_point = EcPoint::from_bytes(&curve, &public_key_bytes, &mut ctx).unwrap();
    let public_key = EcKey::from_public_key(&curve, &public_key_point).unwrap();

    let ecdsa_sig = EcdsaSig::from_der(signature).unwrap();

    ecdsa_sig.verify(data, &public_key).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn signature_verification_success_test() {
        let data = "rcoin".as_bytes();

        let keychain = generate_keychain();
        let public_key = keychain.public_key_hex();
        let signature = keychain.sign(data);

        assert!(verify_signature(data, &signature, &public_key))
    }

    #[test]
    fn signature_verification_success_failure() {
        let data = "rcoin".as_bytes();

        let keychain = generate_keychain();
        let public_key = keychain.public_key_hex();

        let bad_keychain = generate_keychain();
        let signature = bad_keychain.sign(data);

        assert!(!verify_signature(data, &signature, &public_key))
    }
}
