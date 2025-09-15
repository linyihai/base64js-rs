#![deny(clippy::all)]

use base64::{engine::general_purpose::STANDARD, Engine as _};
use napi::bindgen_prelude::Uint8Array;
use napi::bindgen_prelude::*;
use napi::{Error, Status::GenericFailure, Status::InvalidArg};
use napi_derive::napi;

#[napi]
pub fn to_byte_array(input: String) -> Result<Uint8Array> {
  let res = STANDARD
    .decode(input.into_bytes())
    .map_err(|e| Error::new(GenericFailure, format!("decode failed: {}", e)))?;

  Ok(Uint8Array::from(res))
}

#[napi]
pub fn from_byte_array(buf: Uint8Array) -> String {
  STANDARD.encode(buf.as_ref())
}

#[napi]
pub fn byte_length(input: String) -> Result<i64> {
  let len = input.len();
  if len % 4 > 0 {
    return Err(Error::new(
      InvalidArg,
      "Invalid string. Length must be a multiple of 4",
    ));
  }
  let (valid_len, place_holders_len) = {
    let valid_len: usize = input.find("=").unwrap_or(len);
    let place_holders_len = if valid_len == len {
      0
    } else {
      4 - (valid_len % 4)
    };
    (valid_len, place_holders_len)
  };
  let res = ((valid_len + place_holders_len) * 3 / 4) - place_holders_len;

  Ok(res as i64)
}
