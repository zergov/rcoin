pub mod keys {
    use openssl::nid::Nid;
    use openssl::bn::{BigNum, BigNumContext};
    use openssl::ec::{EcPoint, EcGroup, EcKey, PointConversionForm};
    use openssl::hash::{hash, MessageDigest};

    pub struct Keychain {
        private_key: BigNum,
        public_key: EcPoint,
        curve: EcGroup,
    }

    pub fn generate_keychain() -> Keychain {
        let curve = EcGroup::from_curve_name(Nid::SECP256K1).unwrap();

        let key_pair = EcKey::generate(&curve).unwrap();

        Keychain {
            private_key: key_pair.private_key().to_owned().unwrap(),
            public_key: key_pair.public_key().to_owned(&curve).unwrap(),
            curve,
        }
    }

    impl Keychain {
        pub fn private_key_to_hex(&self) -> String {
            self.private_key.to_hex_str().unwrap().to_string()
        }

        pub fn public_key_to_hex(&self) -> String {
            let mut ctx = BigNumContext::new().unwrap();
            let mut x = BigNum::new().unwrap();
            let mut y = BigNum::new().unwrap();

            self.public_key
                .affine_coordinates(&self.curve.as_ref(), &mut x, &mut y, &mut ctx)
                .unwrap();

            format!("04{}{}", x.to_hex_str().unwrap(), y.to_hex_str().unwrap())
        }

        pub fn address(&self) -> String {
            let mut ctx = BigNumContext::new().unwrap();
            let public_key_bytes = self.public_key
                .to_bytes(&self.curve, PointConversionForm::UNCOMPRESSED, &mut ctx)
                .unwrap();

            let payload = hash(MessageDigest::sha256(), &public_key_bytes).unwrap();
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
    }
}
