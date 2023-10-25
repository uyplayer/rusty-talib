/*
 * @Author: uyplayer
 * @Date: 2023/10/25 15:02
 * @Email: uyplayer@qq.com
 * @File: double_exponential_moving_average.rs
 * @Software: RustRover
 * @Dir: rusty-talib / src/overlap_studies
 * @Project_Name: rusty-talib
 * @Description:
 */


//! double exponential moving average

use polars::prelude::*;
use crate::exponential_moving_average;
use std::ops::Mul;

///
/// python code :  2 * ta.ema(Source, Length) - ta.ema(ta.ema(Source, Length), Length)
///
/// ```python
///     close = [35, 10, 20, 56, 89, 76, 30, 46, 10, 653]
///     d = {'close': close}
///     random_data = DataFrame(data=d)
///     print(ta.DEMA(random_data, 3))
/// ```
/// Calculates the Double Exponential Moving Average (DEMA) using the formula in python and ta_lib:
/// `2 * ta.ema(Source, Length) - ta.ema(ta.ema(Source, Length), Length)`
///
/// # Arguments
///
/// * `src` - A Series containing the source data.
/// * `time_period` - An optional parameter representing the time period. Defaults to 5 if not provided.
///
/// # Example
///
/// ```
/// use polars::prelude::*;
/// use rusty_talib::double_exponential_moving_average;
///
/// let result = double_exponential_moving_average(&Series::new("data", vec![1.0, 2.0, 3.0, 4.0, 5.0]), Some(3));
/// eprintln!("{:?}", result);
/// ```
///
/// # Errors
///
/// This function will return an error if there are any issues with the calculations.
///
/// # Returns
///
/// A Result containing a Series of the DEMA values, or an error.
/// # Examples
/// ```
///  use polars::prelude::*;
///  use rusty_talib::double_exponential_moving_average;
///
///  let random_data: [i32; 10] = [35, 10, 20, 56, 89, 76, 30, 46, 10, 653];
///  let close = Series::new("data", random_data);
///  let res = double_exponential_moving_average(&close, Some(3));
///  eprintln!("{:?}", res);
/// ```
///
pub fn double_exponential_moving_average(src:&Series,time_period: Option<usize>)-> Result<Series, Box<dyn std::error::Error>> {

    let time_period = time_period.unwrap_or(5);
    let ema =  exponential_moving_average(src,Some(time_period))?;
    let dema = ema.clone().mul(2) - exponential_moving_average(&ema,Some(time_period))?;
    Ok(Series::new("data", dema))

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_exponential_moving_average() -> Result<(), Box<dyn std::error::Error>> {
        let random_data: [i32; 10] = [35, 10, 20, 56, 89, 76, 30, 46, 10, 653];
        let close = Series::new("data", random_data);
        let res = double_exponential_moving_average(&close, Some(3))?;
        eprintln!("{:?}", res);
        Ok(())
    }
}
