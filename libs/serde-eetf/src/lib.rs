//! Erlang external term format support for serde.
//! This allows a rust app to easily talk to erlang,
//! elixir or any other BEAM language with little boilerplate.
//!
//!```rust
//! #[macro_use]
//! extern crate serde_derive;
//! extern crate serde_eetf;
//!
//! fn main() {
//!     use serde_eetf::{to_bytes, from_bytes};
//!
//!     #[derive(Deserialize, Serialize, PartialEq, Debug)]
//!     struct Data {
//!         token: String,
//!     }
//!
//!     #[derive(Deserialize, Serialize, PartialEq, Debug)]
//!     struct Message {
//!         id: i64,
//!         opcode: u8,
//!         data: Data,
//!     }
//!
//!     let message = Message {
//!         id: 0,
//!         opcode: 5,
//!         data: Data {
//!             token: "SuPeR_SeCrEt_tOkEn".to_string(),
//!         },
//!     };
//!
//!     let encoded = to_bytes(&message).unwrap();
//!     let decoded: Message = from_bytes(&encoded).unwrap();
//!
//!     assert_eq!(decoded, message);
//! }
//! ```

extern crate eetf;
extern crate heck;
extern crate num_bigint;
extern crate num_traits;
#[macro_use]
extern crate serde;

// We need serde_derive to derive serializers/deserializers in our tests.
#[cfg(test)]
#[macro_use]
extern crate serde_derive;

use std::io::Cursor;

use eetf::Term;
use serde_json::Serializer;
use wasm_bindgen::prelude::wasm_bindgen;

pub use crate::de::{Deserializer, from_bytes, from_reader};
pub use crate::error::{Error, Result};
pub use crate::ser::{to_bytes, to_writer};

mod de;
mod error;
mod ser;

#[wasm_bindgen]
pub fn pack(json: &str) -> Vec<u8> {
    let mut buffer = Cursor::new(Vec::new());
    let mut deserializer = serde_json::de::Deserializer::from_str(json);

    let serializer = &ser::Serializer {};
    let result = serde_transcode::transcode(&mut deserializer, serializer).unwrap();

    result.encode(&mut buffer).expect("Failed to encode result");

    buffer.into_inner()
}

#[wasm_bindgen]
pub fn unpack(data: Vec<u8>) -> String {
    let mut writer = Cursor::new(Vec::new());
    let mut reader = Cursor::new(data);

    let mut serializer = Serializer::new(&mut writer);
    let deserializer = Deserializer::new(Term::decode(&mut reader)
        .expect("Failed to decode data"));


    serde_transcode::transcode(deserializer, &mut serializer).unwrap();

    String::from_utf8(writer.into_inner()).expect("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wasm() {
        let json = r#"
        {
            "id": 1,
            "opcode": 5,
            "data":
                {
                    "token": "SuPeR_SeCrEt_tOkEn"
                }
        }
        "#;

        assert_eq!(
            unpack(pack(json)),
            json.chars().filter(|c| !c.is_whitespace()).collect::<String>(),
        );
    }
}
