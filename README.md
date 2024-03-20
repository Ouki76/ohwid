# ohwid

# Example
```rust
use ohwid::get_hwid;
let hwid = get_hwid();

match hwid {
  Ok(hwid) => println!("HWID: {}", hwid),
  Err(e) => println!("Failed to get HWID: {}", e),
}
```
