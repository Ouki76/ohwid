#[cfg(target_os = "windows")]
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
/// Get the hardware ID of the current machine
/// 
/// # Example:
/// ```
/// use ohwid::get_hwid;
/// let hwid = get_hwid();
/// 
/// match hwid {
///     Ok(hwid) => println!("HWID: {}", hwid),
///     Err(e) => println!("Failed to get HWID: {}", e),
/// }
/// ```
pub fn get_hwid() -> std::io::Result<String> {
    let key = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey("SOFTWARE\\Microsoft\\Cryptography")?;

    key.get_value::<String, _>("MachineGuid")
}

#[cfg(not(target_os = "windows"))]
pub fn get_hwid() -> Result<String, std::io::Error> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "get_hwid is only supported on Windows",
    ))
}