#![deny(clippy::all)]

use base64::{engine::general_purpose::STANDARD, Engine as _};
use napi::bindgen_prelude::Uint8Array;
use napi::bindgen_prelude::*;
use napi::{Error, Status::GenericFailure};
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
