use std::prelude::v1::*;

use eth_types::SU256;
use ff::*;

#[derive(PrimeField)]
#[PrimeFieldModulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
#[PrimeFieldGenerator = "7"]
pub struct Fr(FrRepr);

impl From<SU256> for Fr {
    fn from(val: SU256) -> Self {
        Fr(FrRepr(val.0))
    }
}

impl From<Fr> for SU256 {
    fn from(f: Fr) -> Self {
        let mut val = SU256::default();
        val.0 = f.0 .0;
        val
    }
}