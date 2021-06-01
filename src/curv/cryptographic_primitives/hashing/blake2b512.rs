/*
    This file is part of Curv library
    Copyright 2018 by Kzen Networks
    (https://github.com/KZen-networks/curv)
    License MIT: https://github.com/KZen-networks/curv/blob/master/LICENSE
*/

use blake2_rfc::blake2b::Blake2b;
use curv::arithmetic::big_gmp::BigInt;
use curv::arithmetic::traits::Converter;
use curv::elliptic::curves::curve_jubjub::FE;
use curv::elliptic::curves::curve_jubjub::GE;
use curv::elliptic::curves::traits::{ECPoint, ECScalar};

pub struct Blake;

impl Blake {
    pub fn create_hash(big_ints: &[&BigInt], persona: &[u8]) -> BigInt {
        let mut digest = Blake2b::with_params(64, &[], &[], persona);

        for value in big_ints {
            let mut vec = BigInt::to_vec(value);
            digest.update(&vec);
        }
        let ret = digest.finalize();
        BigInt::from(ret.as_ref())
    }

    pub fn create_hash_from_ge(ge_vec: &[&GE], persona: &[u8]) -> FE {
        let mut digest = Blake2b::with_params(64, &[], &[], persona);

        for value in ge_vec {
            digest.update(&value.pk_to_key_slice());
        }

        let result = BigInt::from(digest.finalize().as_ref());
        ECScalar::from(&result)
    }
}

#[cfg(test)]
mod tests {
    use super::Blake;
    use curv::arithmetic::big_gmp::BigInt;
    use curv::elliptic::curves::curve_jubjub::GE;
    use curv::elliptic::curves::traits::ECPoint;
    use curv::elliptic::curves::traits::ECScalar;

    #[test]
    // Very basic test here, TODO: suggest better testing
    fn create_hash_test() {
        let result =
            Blake::create_hash(&vec![&BigInt::one(), &BigInt::zero()], b"Zcash_RedJubjubH");
        assert!(result > BigInt::zero());
    }

    #[test]
    fn create_hash_from_ge_test() {
        let point = GE::base_point2();
        let result1 =
            Blake::create_hash_from_ge(&vec![&point, &GE::generator()], b"Zcash_RedJubjubH");
        assert!(result1.to_big_int().to_str_radix(2).len() > 240);
        let result2 =
            Blake::create_hash_from_ge(&vec![&GE::generator(), &point], b"Zcash_RedJubjubH");
        assert_ne!(result1, result2);
        let result3 =
            Blake::create_hash_from_ge(&vec![&GE::generator(), &point], b"Zcash_RedJubjubH");
        assert_eq!(result2, result3);
    }

}
