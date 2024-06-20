#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use selection::get_text;

#[napi]
pub fn get_selected_text() -> String {
  get_text()
}
