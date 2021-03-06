use std::io::BufRead;

use bincode::{deserialize_from, Result};
use flate2::bufread::ZlibDecoder;
use serde::de::DeserializeOwned;

fn from_reader<T: DeserializeOwned, R: BufRead>(input: R) -> Result<T> {
    let mut decoder = ZlibDecoder::new(input);
    deserialize_from(&mut decoder)
}

fn from_binary<T: DeserializeOwned>(v: &[u8]) -> T {
    from_reader(v).unwrap()
}

#[inline(always)]
pub fn integrated_syntaxset() -> syntect::parsing::SyntaxSet {
    from_binary(include_bytes!("./syntaxes.bin"))
}

#[inline(always)]
pub fn integrated_themeset() -> syntect::highlighting::ThemeSet {
    from_binary(include_bytes!("./themes.bin"))
}
