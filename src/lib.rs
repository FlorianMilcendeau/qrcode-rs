#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use qrcode_generator::QrCodeEcc;
use serde_derive::{Deserialize, Serialize};

#[napi(object)]
pub struct ToSignerInput {
  pub owner: String,
  pub entity: String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct ToSigner {
  owner: String,
  entity: String,
}

#[napi]
pub fn create_qr_code(to_signer_input: ToSignerInput) -> String {
  let to_signer = ToSigner {
    owner: String::from(to_signer_input.owner.as_str()),
    entity: String::from(to_signer_input.entity.as_str()),
  };

  let bytes = bincode::serialize(&to_signer).unwrap();
  let svg = qrcode_generator::to_svg_to_string(bytes, QrCodeEcc::Low, 1024, None::<&str>).unwrap();

  return svg;
}
