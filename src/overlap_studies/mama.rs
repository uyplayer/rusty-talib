/*
 * @Author: uyplayer
 * @Date: 2023/11/6 22:51
 * @Email: uyplayer@qq.com
 * @File: mama.rs
 * @Software: RustRover
 * @Dir: rusty-talib / src/overlap_studies
 * @Project_Name: rusty-talib
 * @Description:
 */


//! MESA Adaptive Moving Average (MAMA)
//! This module provides an implementation of the MESA Adaptive Moving Average indicator.
//! You can use this indicator to calculate the MAMA and FAMA values.
//! For more details on the MAMA indicator, see: https://www.prorealcode.com/prorealtime-indicators/john-ehlers-mama-the-mother-of-adaptive-moving-average/
//!
//! # Example
//!
//! ```
//! use polars::prelude::*;
//! use rusty_talib::mama;
//! // Create sample data
//! let high = Series::new("High", [1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 0, 11, 12, 13]);
//! let close = Series::new("Close", [1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 0, 11, 12, 13]);
//!
//! // Calculate MAMA and FAMA
//! let mama = mama(&high, &close).expect("Failed to calculate MAMA");
//!
//! // Print the MAMA values
//! println!("{:?}", mama);
//! ```
//!
//! Note: You need to have the `polars` crate in your `Cargo.toml` for this to work.
//!
//! # Details
//!
//! This implementation calculates the MAMA and FAMA values using the provided algorithm.

use polars::export::arrow::array::Float64Array;
use std::ops::Div;
use polars::prelude::*;

/// Calculate the MAMA and FAMA values based on the provided algorithm.
/// This function takes two Series, 'high' and 'low', as input and returns a Series with the MAMA values.
///
/// # Arguments
///
/// * `high` - A Series containing high price data.
/// * `low` - A Series containing low price data.
///
/// # Example
///
/// ```
/// use polars::prelude::*;
/// use rusty_talib::mama;
/// let high = Series::new("High", [1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 0, 11, 12, 13]);
/// let close = Series::new("Close", [1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 0, 11, 12, 13]);
/// let mama_series = mama(&high, &close).expect("Failed to calculate MAMA");
/// ```
pub fn mama<'a>(high: &'a Series, low:&'a Series) ->Result<Series,Box<dyn std::error::Error>> {
    let high = high.cast(&DataType::Float64)?.clone().into_series();
    let low = low.cast(&DataType::Float64)?.clone().into_series();
    let prices =  (high+low).div(2);
    let array = prices.to_arrow(0);
    let prices = match array.as_any().downcast_ref::<Float64Array>() {
        Some(float_array) => {
            let values: &[f64] = float_array.values();
            let vec_values: Vec<f64> = values.to_vec();
            vec_values
        }
        None => return Err("Failed to downcast to Float64Array".into()),
    };

    let fast_limit = 0.5;
    let slow_limit = 0.05;

    let  length :usize= prices.len();
    let mut mama =  vec![0.0;length];
    let mut fama =  vec![0.0;length];
    let mut smooth = vec![0.0;length];
    let mut detrender = vec![0.0;length];
    let mut period = vec![0.0;length];
    let mut q1 = vec![0.0;length];
    let mut i1 = vec![0.0;length];
    let mut ji = vec![0.0;length];
    let mut jq = vec![0.0;length];
    let mut i2 = vec![0.0;length];
    let mut q2 = vec![0.0;length];
    let mut re = vec![0.0;length];
    let mut im = vec![0.0;length];
    let mut smooth_period = vec![0.0;length];
    let mut phase = vec![0.0;length];
    for i in 6..prices.len(){

        smooth[i] = (4.0 * prices[i]
            + 3.0 * smooth[i - 1]
            + 2.0 * smooth[i - 2]
            + smooth[i - 3])
            / 10.0;

        detrender[i] = (0.0962 * smooth[i] + 0.5769 * smooth[i - 2]
            - 0.5769 * smooth[i - 4]
            - 0.0962 * smooth[i - 6])
            * (0.075 * period[i - 1] + 0.54);
        q1[i] = (0.0962 * detrender[i] + 0.5769 * detrender[i - 2]
            - 0.5769 * detrender[i - 4]
            - 0.0962 * detrender[i - 6])
            * (0.075 * period[i - 1] + 0.54);
        i1[i] = detrender[i - 3];
        ji[i] = (0.0962 * i1[i] + 0.5769 * i1[i - 2]
            - 0.5769 * i1[i - 4]
            - 0.0962 * i1[i - 6])
            * (0.075 *  period[i - 1] + 0.54);

        jq[i] = (0.0962 * q1[i] + 0.5769 * q1[i - 2]
            - 0.5769 * q1[i - 4]
            - 0.0962 * q1[i - 6])
            * (0.075 * period[i - 1] + 0.54);

        i2[i] = i1[i] - jq[i];
        q2[i] = q1[i] + ji[i];

        i2[i] = 0.2 * i2[i] + 0.8 * i2[i - 1];
        q2[i] = 0.2 * q2[i] + 0.8 * q2[i - 1];

        re[i] = i2[i] * i2[i - 1] + q2[i] * q2[i - 1];
        im[i] = i2[i] * q2[i - 1] - q2[i] * i2[i - 1];

        re[i] = 0.2 * re[i] + 0.8 * re[i - 1];
        im[i] = 0.2 * im[i] + 0.8 * im[i - 1];


        if im[i] != 0.0 && re[i] != 0.0 {
            period[i] = 360.0 / (im[i] / re[i]).atan();
        }

        if period[i] > 1.5 * period[i - 1] {
            period[i] = 1.5 * period[i - 1];
        }

        if period[i] < 0.67 * period[i - 1] {
            period[i] = 0.67 * period[i - 1];
        }
        if period[i] < 6.0 {
            period[i] = 6.0;
        }

        if period[i] > 50.0 {
            period[i] = 50.0;
        }


        period[i] = 0.2 * period[i] + 0.8 * period[i - 1];
        smooth_period[i] = 0.33 * period[i] + 0.67 * smooth_period[i - 1];
        if i1[i] != 0.0 {
            phase[i] = q1[i] / i1[i];
        }

        let mut delta_phase = phase[i - 1] - phase[i];

        if delta_phase < 1.0 {
            delta_phase = 1.0;
        }

        let mut alpha = fast_limit / delta_phase;

        if alpha < slow_limit {
            alpha = slow_limit;
        }

        mama[i] = alpha * prices[i] + (1.0 - alpha) * mama[i - 1];
        fama[i] = 0.5 * alpha * mama[i] + (1.0 - 0.5 * alpha) * fama[i - 1];

    }
    Ok(Series::new("data",mama))
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mama() ->Result<(), Box<dyn std::error::Error>>{

        let high = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 0, 11, 12, 13,
        ];
        let close = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 0, 11, 12, 13,
        ];
        // let mut prices = Series::new("data", close);
        // let prices = prices.f64().unwrap().set_at_idx(vec![0, 1], Some(10.10)).unwrap().into_series();
        // eprintln!("{}",prices);
        let ma = mama(&Series::new("data",high), &Series::new("data", close))?;
        eprintln!("{}",ma);
        Ok(())

    }
}
