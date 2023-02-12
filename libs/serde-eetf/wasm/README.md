# Garthen Serde EETF Library

Erlang external term format support for serde.
This allows a rust app to easily talk to erlang, elixir or any other BEAM language with little boilerplate.

## Usage

### Rust

Add to project

```toml
[dependencies]
serde_derive = "1.0"
serde-eetf = { path = "@/libs/serde-eetf" }
```

> You can also use `serde` with the `derive` feature instead of `serde_derive`

Write some Rust

```rust
#[macro_use]
extern crate serde_derive;
extern crate serde_eetf;

fn main() {
    use serde_eetf::{to_bytes, from_bytes};

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    struct Data {
        token: String,
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    struct Message {
        id: i64,
        opcode: u8,
        data: Data,
    }

    let message = Message {
        id: 0,
        opcode: 5,
        data: Data {
            token: "SuPeR_SeCrEt_tOkEn".to_string(),
        },
    };

    let encoded = to_bytes(&message).unwrap();
    let decoded: Message = from_bytes(&encoded).unwrap();

    assert_eq!(decoded, message);
}
```

### Wasm (JS)

Add to project

```json
{
  "dependencies": {
    "serde-eetf": "@/libs/serde-eetf/wasm"
  }
}
```

Write some JavaScript

```js
import init, { pack, unpack } from "serde-eetf";

init().then(() => {
    const message = {
        id: 0,
        opcode: 5,
        data: {
            token: 'SuPeR_SeCrEt_tOkEn',
        },
    }
    
    const encoded = pack(JSON.stringify(message))
    const decoded = unpack(encoded)
    
    if (JSON.parse(decoded).id === message.id) {
        console.log('It worked!')
    }
});
```
