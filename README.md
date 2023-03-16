# DHL Rust API

#
## Supported APIs
[x] DHL
[ ] UPS
[ ] Fedex

### Usage

```rust
fn main() {
  let mut dhl = logistics_api::DHL::new("api-key");

  match dhl.tracking.get_tracking("tracking-code") {
    Ok(details) => println!("{:?}", details),
    Err(err) => println!("Something went wrong, {:?}", err)
  }
}
```