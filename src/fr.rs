use std::prelude::v1::*;

use eth_types::SU256;
use ff::*;

#[derive(PrimeField)]
#[PrimeFieldModulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
#[PrimeFieldGenerator = "7"]
pub struct Fr(FrRepr);

use ff::PrimeField;

impl Fr {
    pub fn as_u256(self) -> SU256 {
        self.into()
    }
}

impl From<SU256> for Fr {
    fn from(val: SU256) -> Self {
        Fr::from_repr(FrRepr(val.0)).unwrap()
    }
}

impl From<Fr> for SU256 {
    fn from(f: Fr) -> Self {
        let repr = f.into_repr();
        let required_length = repr.as_ref().len() * 8;
        let mut buf: Vec<u8> = Vec::with_capacity(required_length);
        repr.write_be(&mut buf).unwrap();
        SU256::from_big_endian(&buf)
    }
}