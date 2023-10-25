/*
 * @Author: uyplayer
 * @Date: 2023/10/22 12:18
 * @Email: uyplayer@qq.com
 * @File: bbands.rs
 * @Software: RustRover
 * @Dir: rusty-talib / src/overlap_studies
 * @Project_Name: rusty-talib
 * @Description:
 */

use crate::simple_moving_average;
use crate::ErrorMsg;
use polars::prelude::*;
use std::ops::Mul;

///
///``` python
/// # python
///     random.seed()
///     methods_and_attributes = dir(ta)
///     print(methods_and_attributes)
///     close = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 1.0, 0.0, 11.0, 12.0, 13.0]
///     d = {'close': close}
///     random_data = DataFrame(data=d)
///     print(ta.BBANDS(random_data, 5, 2.0))
///
///     upperband  middleband  lowerband
/// 0         NaN         NaN        NaN
/// 1         NaN         NaN        NaN
/// 2         NaN         NaN        NaN
/// 3         NaN         NaN        NaN
/// 4    5.828427         3.0   0.171573
/// 5    6.828427         4.0   1.171573
/// 6    7.828427         5.0   2.171573
/// 7    8.828427         6.0   3.171573
/// 8    9.828427         7.0   4.171573
/// 9   11.771355         6.2   0.628645
/// 10  12.483315         5.0  -2.483315
/// 11  14.690444         5.8  -3.090444
/// 12  16.766612         6.6  -3.566612
/// 13  18.756056         7.4  -3.956056
///
/// ```
/// Calculates the Bollinger Bands (BBands) based on the provided closing prices and parameters.
///
/// # Arguments
///
/// * `close` - A reference to a vector of closing prices for the financial instrument.
/// * `time_period` - An optional time period used in the calculations. Default is 14.
/// * `multi` - An optional multiplier value used in the calculations. Default is 5.
///
/// # Returns
///
/// Returns a tuple containing three Series objects representing the middle band, upper band, and lower band of the Bollinger Bands.
///
/// # Errors
///
/// Returns an error message if the length of the `close` vector is less than the specified time period.
///
/// # Examples
/// ```
/// use polars::prelude::*;
/// use rusty_talib::b_bands;
/// use rusty_talib::ErrorMsg;
/// let close = Vec::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 1.0, 0.0, 11.0, 12.0, 13.0]);
/// let res = b_bands(&Series::new("data",close), Some(5), Some(2));
///     match res {
///             Ok((middle_band, upper_band, lower_band)) => {
///                 eprintln!("{:?}", middle_band);
///                 eprintln!("{:?}", upper_band);
///                 eprintln!("{:?}", lower_band);
///             }
///             Err(e) => {
///                 if let Some(my_error) = e.downcast_ref::<ErrorMsg>() {
///                     eprintln!("{}", my_error.0);
///                 } else {
///                     eprintln!("An error occurred");
///                 }
///             }
///         }
/// ```
///
pub fn b_bands<'a>(
    close: &Series,
    time_period: Option<usize>,
    multi: Option<usize>,
) -> Result<(Series, Series, Series), Box<dyn std::error::Error>> {
    let time_period = time_period.unwrap_or(14);
    let multi = multi.unwrap_or(5);

    if close.len() < time_period {
        return Err(Box::new(ErrorMsg(
            "src Length must be greater than time_period".into(),
        )));
    }

    let basis = simple_moving_average(&close, Some(time_period as usize))?;

    let duration = Duration::new(time_period as i64);
    let options = RollingOptionsImpl {
        window_size: duration,
        min_periods: 1,
        ..Default::default()
    };
    let dev = close.rolling_std(options)?;
    let dev = dev.mul(multi);
    let middle_band = &basis;
    let upper_band = basis.add_to(&dev)?;
    let lower_band = basis.subtract(&dev)?;
    Ok((middle_band.clone(), upper_band.clone(), lower_band.clone()))
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_b_bands() {
        // let mut rng = rand::thread_rng();
        // let close: Vec<f64> = (0..1000).map(|_| rng.gen_range(1.0..2000.0)).collect();
        let close = Vec::from([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 1.0, 0.0, 11.0, 12.0, 13.0,
        ]);
        let res = b_bands(&Series::new("data",close), Some(5), Some(2));
        match res {
            Ok((middle_band, upper_band, lower_band)) => {
                eprintln!("{:?}", middle_band);
                eprintln!("{:?}", upper_band);
                eprintln!("{:?}", lower_band);
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
