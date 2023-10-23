/*
 * @Author: uyplayer
 * @Date: 2023/10/22 19:30
 * @Email: uyplayer@qq.com
 * @File: exponential_moving_average
 * @Software: RustRover
 * @Dir: rusty-talib / src/overlap_studies
 * @Project_Name: rusty-talib
 * @Description:
 */

//! exponential_moving_average

use crate::ErrorMsg;
use polars::prelude::*;


/// Calculates the exponential moving average of a Series.
///
/// ``` python
///     #  This Python code produces the same result as the exponential_moving_average function does
///     import talib.abstract as ta
///     from pandas import DataFrame
///     random_data = [35, 10, 20, 56, 89, 76, 30, 46, 10, 653]
///     d = {'close': random_data}
///     random_data = DataFrame(data=d)
///     res = ta.EMA(random_data, 3)
///     print(res)
///     print(random_data.ewm(span=3,adjust=False).mean())
/// ```
///
/// # Arguments
///
/// * `src` - A reference to the Series for which the exponential moving average needs to be calculated.
/// * `time_period` - An optional parameter specifying the time period for the exponential moving average. If not provided, the default value is 14.
///
/// # Errors
///
/// Returns an error if the length of the `src` Series is less than the `time_period` or if there is an issue with data extraction.
///
/// # Examples
///
/// ```
/// use polars::prelude::*;
/// use rusty_talib::exponential_moving_average;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let series = Series::new("data", values);
///
/// let result = exponential_moving_average(&series, Some(3));
/// match result {
///    Ok(ema) => {
///         println!("Exponential Moving Average: {:?}", ema);
///         }
///    Err(err) => {
///         eprintln!("Error: {}", err);
///         }
///     }
///     Ok(())
///  }
/// ```
///
/// This function calculates the exponential moving average for the specified `time_period`
/// using the formula for calculating EMA. The `src` Series must have a length greater than or equal to the `time_period`.
pub fn exponential_moving_average<'a>(src: &'a Series, time_period: Option<u32>) -> Result<Series, Box<dyn std::error::Error>> {
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
    let alpha = 2.0 / (time_period as f64 + 1.0);
    let mut ema_values: Vec<f64> = vec![];
    for i in 0..src.len() {
        let elem = src.get(i)?.try_extract::<f64>()?;
        if i == 0 {
            ema_values.push(elem);
            continue;
        }
        let ema = alpha * elem + (1.0 - alpha) * ema_values[i - 1];
        ema_values.push(ema);
    }
    Ok(Series::new("data", ema_values))
}


// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_b_bands() -> Result<(), Box<dyn std::error::Error>> {
        let random_data: [i32; 10] = [35, 10, 20, 56, 89, 76, 30, 46, 10, 653];
        let close = Series::new("data", random_data);
        let res = exponential_moving_average(&close, Some(3))?;
        eprintln!("{:?}", res);
        Ok(())
    }
}