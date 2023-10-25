/*
 * @Author: uyplayer
 * @Date: 2023/10/24 18:36
 * @Email: uyplayer@qq.com
 * @File: simple_moving_average.rs
 * @Software: RustRover
 * @Dir: rusty-talib / src/overlap_studies
 * @Project_Name: rusty-talib
 * @Description:
 */

use crate::ErrorMsg;
use polars::prelude::*;

/// Calculates the simple moving average within the given time period.
///
///
/// # Arguments
///
/// * `src` - A Series object containing the data.
/// * `time_period` - An optional time window size for calculating the moving average. If the time window size is not provided, the default is 14.
///
/// # Returns
///
/// Returns a new Series object containing the calculated simple moving average.
///
/// # Errors
///
/// Returns an error message if the length of the source Series is less than the specified time window size.
///
/// # Examples
///
/// ```
/// use polars::prelude::*;
/// use rusty_talib::simple_moving_average;
/// use rusty_talib::ErrorMsg;
///
/// let random_data: [i32; 7] = [23, 25, 12, 28, 33, 31, 35];
/// let close = Series::new("data", random_data);
/// let res = simple_moving_average(&close, Some(3));
///  match res {
///     Ok(res) => {
///         assert_eq!(res.len(), close.len());
///         eprintln!("{:?}", res);
///     }
///     Err(e) => {
///         if let Some(my_error) = e.downcast_ref::<ErrorMsg>() {
///            eprintln!("{}", my_error.0);
///          } else {
///             eprintln!("An error occurred");
///          }
///     }
/// }
/// ```
///
pub fn simple_moving_average<'a>(
    src: &'a Series,
    time_period: Option<usize>,
) -> Result<Series, Box<dyn std::error::Error>> {
    let time_period = time_period.unwrap_or(14);
    if src.len() < time_period {
        return Err(Box::new(ErrorMsg(
            "src Length must be greater than time_period".into(),
        )));
    }

    let duration = Duration::new(time_period as i64);
    let options = RollingOptionsImpl {
        window_size: duration,
        min_periods: 1,
        weights: None,
        center: false,
        by: None,
        tu: None,
        tz: None,
        closed_window: None,
        fn_params: None,
    };

    let res = src.rolling_mean(options)?;
    Ok(res)
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_b_bands() {
        let random_data: [i32; 7] = [23, 25, 12, 28, 33, 31, 35];
        let close = Series::new("data", random_data);
        let res = simple_moving_average(&close, Some(3));
        match res {
            Ok(res) => {
                assert_eq!(res.len(), close.len());
                eprintln!("{:?}", res);
            }
            Err(e) => {
                if let Some(my_error) = e.downcast_ref::<ErrorMsg>() {
                    eprintln!("{}", my_error.0);
                } else {
                    eprintln!("An error occurred");
                }
            }
        }
    }
}
