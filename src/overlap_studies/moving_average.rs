/*
 * @Author: uyplayer
 * @Date: 2023/10/22 15:59
 * @Email: uyplayer@qq.com
 * @File: moving_average
 * @Software: RustRover
 * @Dir: rusty-talib / src/overlap_studies
 * @Project_Name: rusty-talib
 * @Description:
 */

//! moving average

use crate::ErrorMsg;
use polars::prelude::*;



/// Calculates the moving average of a given series.
///
/// # Arguments
///
/// * `src` - A reference to the Series on which the moving average is to be calculated.
/// * `time_period` - An optional parameter representing the time period for the moving average calculation. If not provided, the default value is 14.
///
/// # Examples
///
/// ```
/// use polars::prelude::*;
/// use rusty_talib::overlap_studies::moving_average;
/// use rusty_talib::ErrorMsg;
///
/// fn main() {
///     let data = Series::new("data", &[1, 2, 3, 4, 5]);
///     let result = moving_average(&data, Some(2));
///     match result {
///         Ok(ma) => {
///             println!("{:?}", ma);
///         },
///         Err(e) => {
///             if let Some(error) = e.downcast_ref::<ErrorMsg>() {
///                 println!("{}", error.0);
///             } else {
///                 println!("An error occurred");
///             }
///         }
///     }
/// }
/// ```
///
pub fn moving_average<'a >(src: &'a Series, time_period: Option<u32>) -> Result<Series, Box<dyn std::error::Error>> {
    let time_period = match time_period {
        Some(p) => p,
        None => 14,
    };
    let time_period = time_period as usize;
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
    use rand::Rng;
    use super::*;

    #[test]
    fn test_b_bands() {
        let random_data: [i32; 7] = [23, 25, 12, 28, 33, 31, 35];
        let close = Series::new("data",random_data);
        let res = moving_average(&close,Some(3));
        match res {
            Ok(ma) => {
                assert_eq!(ma.len(),close.len());
                eprintln!("{:?}",ma);
            },
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