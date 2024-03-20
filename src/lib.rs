#[cfg(target_os = "windows")]
mod hwid {
    use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

    pub fn get_hwid() -> std::io::Result<String> {
        let key = RegKey::predef(HKEY_LOCAL_MACHINE)
            .open_subkey("SOFTWARE\\Microsoft\\Cryptography")?;
        
        key.get_value::<String, _>("MachineGuid")
    }
}

#[cfg(target_os = "linux")]
mod hwid {
    pub fn get_hwid() -> std::io::Result<String> {
        Ok(std::fs::read_to_string("/etc/machine-id")?.trim_end_matches("\n").to_string())
    }
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub fn get_hwid() -> std::io::Result<String> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "get_hwid is only supported on Windows",
    ))
}

/// Get the hardware ID of the current machine
/// 
/// # Example
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
    hwid::get_hwid()
}