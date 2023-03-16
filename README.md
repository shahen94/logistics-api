# DHL Rust API

### Usage

```rust
fn main() {
  let mut dhl = dhl_api::DHL::new("api-key");

  match dhl.tracking.get_tracking("tracking-code") {
    Ok(details) => println!("{:?}", details),
    Err(err) => println!("Something went wrong, {:?}", err)
  }
}
```