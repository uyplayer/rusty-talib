/*
 * @Author: uyplayer
 * @Date: 2023/11/8 7:49
 * @Email: uyplayer@qq.com
 * @File: mavp.rs
 * @Software: RustRover
 * @Dir: rusty-talib / src/overlap_studies
 * @Project_Name: rusty-talib
 * @Description:
 */

//! Calculate Moving Average with Variable Period (MAVP).
//!
//! This function computes the Moving Average with Variable Period (MAVP) for a given input Series of price data.
//! MAVP is calculated for different variable periods, and the result is returned as a Polars Series.
//!
//! # Arguments
//!
//! * `src` - A reference to the input Series containing price data.
//! * `periods` - An optional list of variable periods for MAVP. If not provided, default periods [2, 5, 8] will be used.
//! * `min_period` - An optional parameter specifying the minimum allowed period for MAVP. If not provided, a default value of 2 will be used.
//! * `max_period` - An optional parameter specifying the maximum allowed period for MAVP. If not provided, a default value of 30 will be used.
//!
//! # Example
//!
//! ```rust
//! use polars::prelude::*;
//! use rusty_talib::mavp;
//!
//! // Create a Series with price data
//! let prices = Series::new("price", &[50.0, 52.0, 55.0, 57.0, 60.0]);
//!
//! // Calculate MAVP with custom periods and parameters
//! let mavp_result = mavp(&prices, Some(vec![2, 3, 4]), Some(2), Some(10));
//!
//! // Print the MAVP values
//! println!("{:?}", mavp_result);
//! ```
//!
//! # Returns
//!
//! A Polars Series containing the MAVP values for the specified periods.
//!
//! # Errors
//!
//! Returns an error if there is an issue with data conversion or other errors during execution.
//!
//! # Note
//!
//! This implementation uses the Polars library for data manipulation and computation.
//!
//! # See Also
//!
//! - [simple_moving_average](crate::simple_moving_average) - A related function for simple moving average.
//!
//! # References
//!
//! - [MAVP Documentation](https://btalib.backtrader.com/indgroups/)
//!

use polars::export::arrow::array::Float64Array;
use polars::prelude::*;

/// Calculate Moving Average with Variable Period (MAVP).
///
/// # Arguments
///
/// * `src` - A reference to the input Series containing price data.
/// * `periods` - An optional list of variable periods for MAVP. If not provided, default periods [2, 5, 8] will be used.
/// * `min_period` - An optional parameter specifying the minimum allowed period for MAVP. If not provided, a default value of 2 will be used.
/// * `max_period` - An optional parameter specifying the maximum allowed period for MAVP. If not provided, a default value of 30 will be used.
///
/// # Returns
///
/// A Polars Series containing the MAVP values for the specified periods.
///
/// # Errors
///
/// Returns an error if there is an issue with data conversion or other errors during execution.
///
/// # Note
///
/// This implementation uses the Polars library for data manipulation and computation.
///
/// # See Also
///
/// - [simple_moving_average](crate::simple_moving_average) - A related function for simple moving average.
///
/// # References
///
/// - [MAVP Documentation](https://btalib.backtrader.com/indgroups/)
///

pub fn mavp<'a>(
    src: &'a Series,
    periods: Option<Vec<i32>>,
    min_period: Option<usize>,
    max_period: Option<usize>,
) -> Result<Series, Box<dyn std::error::Error>> {

    let src = src.cast(&DataType::Float64)?.clone().into_series();
    let periods = periods.unwrap_or(vec![2, 5, 8]);
    let min_period = min_period.unwrap_or(2);
    let max_period = max_period.unwrap_or(30);

    assert_eq!(src.len(),periods.iter().len(),"Lengths are not same");

    let mut mavp_values: Vec<f64> = Vec::new();
    let array = src.to_arrow(0);
    let prices = match array.as_any().downcast_ref::<Float64Array>() {
        Some(float_array) => {
            let values: &[f64] = float_array.values();
            let vec_values: Vec<f64> = values.to_vec();
            vec_values
        }
        None => return Err("Failed to downcast to Float64Array".into()),
    };
    for (i, &period) in periods.iter().enumerate() {
        if period < min_period as i32 || period > max_period as i32 {
            mavp_values.push(0.0);
        } else {
            let mut sum = 0.0;
            for j in i.saturating_sub(period as usize)..=i {
                sum += prices[j];
            }
            let mavp = sum / period as f64;
            mavp_values.push(mavp);
        }
    }
    Ok(Series::new("data", &mavp_values))
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponential_moving_average() -> Result<(), Box<dyn std::error::Error>> {
        let random_data: [i32; 10] = [35, 10, 20, 56, 89, 76, 30, 46, 10, 653];
        let close = Series::new("data", random_data);
        let res = mavp(&close, Option::from(vec![2, 5, 8,1,6,9,4,2,3,1]), Some(2), Some(8))?;
        eprintln!("{:?}", res);
        Ok(())
    }
}
