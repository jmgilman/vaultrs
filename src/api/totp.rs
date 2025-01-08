use serde::{Deserialize, Serialize};

pub mod requests;
pub mod responses;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Alg {
    Sha1,
    Sha256,
    Sha512,
}

impl Default for Alg {
    fn default() -> Self {
        Alg::Sha1
    }
}

pub enum Digits {
    Six = 6,
    Eight = 8,
}

impl Default for Digits {
    fn default() -> Self {
        Digits::Six
    }
}

pub enum Skew {
    Zero = 0,
    One = 1,
}

impl Default for Skew {
    fn default() -> Self {
        Skew::One
    }
}
