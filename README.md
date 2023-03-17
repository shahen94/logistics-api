# Logistics API

## Supported APIs
* DHL   (✅)
* UPS   (🕒)
* Fedex (🕒)

### Usage

**With Blocking I/O**

```rust
use logistics_api::DHL;

fn main() {
  let dhl = DHL::new("YOUR_API_KEY");

  let tracking = dhl.tracking.get_tracking_sync("YOUR_TRACKING_NUMBER");

  match tracking {
    Ok(tracking) => println!("{:#?}", tracking),
    Err(err) => println!("{:#?}", err),
  }
}
```

**Using Async I/O**

```rust
use logistics_api::DHL;

#[tokio::main]
async fn main() {
  let dhl = DHL::new("YOUR_API_KEY");

  let tracking = dhl.tracking.get_tracking("YOUR_TRACKING_NUMBER").await;

  match tracking {
    Ok(tracking) => println!("{:#?}", tracking),
    Err(err) => println!("{:#?}", err),
  }
}
```