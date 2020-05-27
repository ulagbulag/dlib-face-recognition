use std::ffi::*;
use std::path::*;

pub fn path_as_cstring(path: &Path) -> Result<CString, String> {
    if !path.exists() {
        Err(format!("File not found: '{}'", path.display()))
    } else {
        let string = path.to_str().unwrap();
        Ok(CString::new(string).unwrap())
    }
}
