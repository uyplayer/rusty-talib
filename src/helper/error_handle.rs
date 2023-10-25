/*
 * @Author: uyplayer
 * @Date: 2023/10/24 20:13
 * @Email: uyplayer@qq.com
 * @File: error_handle
 * @Software: RustRover
 * @Dir: rusty-talib / src/helper
 * @Project_Name: rusty-talib
 * @Description:
 */

use std::error::Error;
use std::fmt;

/// Custom error type for providing error messages.
#[derive(Debug)]
pub struct ErrorMsg(pub String);

impl fmt::Display for ErrorMsg {
    /// Formats the error message.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ErrorMsg {}
