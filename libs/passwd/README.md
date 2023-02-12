# Password Library

Pure Rust implementation of the [Argon2](https://en.wikipedia.org/wiki/Argon2) password hashing function.

## Usage

Add to project

```toml
[dependencies]
passwd = { path = "@/libs/passwd" }
```

Write some Rust

```rust
fn main() {
    let password = "SuPeR SeCrEt pAsSw0rD";
    let password_hash = passwd::hash(password).expect("Failed to hash password");
    
    passwd::verify(password, password_hash).expect("Entered password is incorrect");
}
```
