
pub mod res;
pub mod compression;

use std::path::PathBuf;

use crate::TEMP_DIR_PATH;
/// 获取临时目录
pub fn get_temp_path() -> PathBuf {
     TEMP_DIR_PATH
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .to_owned()
}



use std::ptr;
use winapi::um::processthreadsapi::OpenProcessToken;
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::winnt::{
    TokenElevation,  TOKEN_ELEVATION, TOKEN_QUERY,
};
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::errhandlingapi::GetLastError;
pub fn is_user_an_admin() -> Result<bool, u32> {
    unsafe {
        let mut token_handle = ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle) == 0 {
            return Err(GetLastError());
        }

        let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut return_length = 0;

        if GetTokenInformation(
            token_handle,
            TokenElevation,
            &mut elevation as *mut _ as *mut _,
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut return_length,
        ) == 0
        {
            return Err(GetLastError());
        }

        let result = elevation.TokenIsElevated != 0;
        Ok(result)
    }
}