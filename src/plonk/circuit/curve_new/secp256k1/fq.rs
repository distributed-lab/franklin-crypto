use crate::bellman::pairing::ff::*;


// base field, Q = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F
#[derive(PrimeField)]
#[PrimeFieldModulus = "115792089237316195423570985008687907853269984665640564039457584007908834671663"]
#[PrimeFieldGenerator = "2"]
pub struct Fq(FqRepr);