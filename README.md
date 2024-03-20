# ohwid
Allows you to get hardware ID of the current machine.

## Supported systems
+ Windows

## Install
```bash
cargo add ohwid
```
or
```toml
[dependencies]
winreg = "0.1.0"
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
