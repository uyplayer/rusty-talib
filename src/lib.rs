
use std::error::Error;
use std::fmt;


#[cfg(feature = "overlap_studies")]
mod overlap_studies;
pub use overlap_studies::{moving_average,exponential_moving_average};





///
/// error handle
///
#[derive(Debug)]
pub struct ErrorMsg(pub String);

impl fmt::Display for ErrorMsg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ErrorMsg {}