use crate::utils::constant::*;
use std::path::PathBuf;

pub fn get_config_path() -> Option<String> {
    // let dir = get_config_local_dir()?;
    // let identifier_path = dir.join(PACKAGE_IDENTIFIER);
    // if !identifier_path.exists() {
    //     std::fs::create_dir_all(&identifier_path).ok()?;
    // }
    // Some(
    //     identifier_path
    //         .join("data.db")
    //         .to_string_lossy()
    //         .into_owned(),
    // )
    None
}

pub fn get_config_local_dir() -> Option<PathBuf> {
    #[cfg(target_os = "android")]
    {
        // use std::ffi::CStr;
        // use std::os::unix::ffi::OsStrExt;

        // let uid = unsafe { libc::getuid() };
        // let pwd = unsafe { libc::getpwuid(uid) };
        // if pwd.is_null() {
        //     return None;
        // }

        // let home_dir = unsafe { CStr::from_ptr((*pwd).pw_dir) };
        // let path = std::path::OsStr::from_bytes(home_dir.to_bytes());
        // return Some(PathBuf::from(path).join("files"));
        return None;
    }
    #[cfg(target_os = "ios")]
    {
        Some(
            PathBuf::from(std::env::var("HOME").ok()?)
                .join("Library")
                .join("Application Support"),
        )
    }
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        dirs::data_local_dir()
    }
}
