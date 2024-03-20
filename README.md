# ohwid
Allows you to get hardware ID of the current machine.

## Supported systems
+ Windows
+ Linux

## Install
```bash
cargo add ohwid
```
or
```toml
[dependencies]
ohwid = "0.1.2"
```

## Example
```rust
use ohwid::get_hwid;
let hwid = get_hwid();

match hwid {
  Ok(hwid) => println!("HWID: {}", hwid),
  Err(e) => println!("Failed to get HWID: {}", e),
}
```
