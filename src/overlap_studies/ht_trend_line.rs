/*
 * @Author: uyplayer
 * @Date: 2023/10/31 14:28
 * @Email: uyplayer@qq.com
 * @File: ht_trend_line
 * @Software: RustRover
 * @Dir: rusty-talib / src/overlap_studies
 * @Project_Name: rusty-talib
 * @Description:
 */


//! Calculation for Hilbert Transform - Instantaneous Trendline
//!
//! This module provides a function for calculating the Hilbert Transform Instantaneous Trendline.
//!
//! The Hilbert Transform Instantaneous Trendline is a mathematical calculation used in technical analysis.
//! It is commonly used in financial markets to analyze price data.
//!
//! For more information on the algorithm and its usage, you can refer to the following sources:
//!
//! - [ProRealCode - John Ehlers Instantaneous Trendline](https://www.prorealcode.com/prorealtime-indicators/john-ehlers-instantaneous-trendline/)
//! - [MQL5 Forum - John Ehlers Instantaneous Trendline](https://c.mql5.com/forextsd/forum/59/023inst.pdf)
//! - [TradingView Script - Blackcat L2 Ehlers Hilbert Transform](https://tw.tradingview.com/script/dFWImthM-blackcat-L2-Ehlers-Hilbert-Transform/)
//!

use std::ops::{Div};
use polars::export::arrow::array::Float64Array;
use polars::prelude::*;
use crate::ErrorMsg;


/// Calculate the Hilbert Transform Instantaneous Trendline.
/// https://www.prorealcode.com/prorealtime-indicators/john-ehlers-instantaneous-trendline/
///
/// https://c.mql5.com/forextsd/forum/59/023inst.pdf
///
/// https://tw.tradingview.com/script/dFWImthM-blackcat-L2-Ehlers-Hilbert-Transform/
///
/// This function takes high and low price data as input and calculates the Hilbert Transform Instantaneous Trendline.
///
/// # Arguments
///
/// * `high` - A series representing high price data.
/// * `low` - A series representing low price data.
///
/// # Returns
///
/// A tuple containing two Series: the Quadrature phase output signal (Q1) and the In-phase output signal (I1).
///
/// # Errors
///
/// Returns an error if the length of the input data is less than 6.
///
/// # Example
///
/// ```
/// use polars::prelude::LiteralValue::Series;
/// use rusty_talib::ht_trend_line;
///
/// // Provide high and low price data
/// let high = Vec::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 1.0, 0.0, 11.0, 12.0, 13.0]);
/// let low = Vec::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 1.0, 0.0, 11.0, 12.0, 13.0]);
///
/// // Calculate the Hilbert Transform Instantaneous Trendline
/// let (q1, i1) = ht_trend_line(&Series::new("data", high), &Series::new("data", low)).unwrap();
///
/// // Print the results
/// eprintln!("{:?}", q1);
/// eprintln!("{:?}", i1);
/// ```

pub fn ht_trend_line<'a>(high: &'a Series, low: &'a Series) -> Result<(Series,Series), Box<dyn std::error::Error>> {
    const BAR_INDEX_6: usize = 6;
    if low.len() < BAR_INDEX_6 || high.len() < BAR_INDEX_6 {
        return Err(Box::new(ErrorMsg(
            "source Length must be greater than 5".into(),
        )));
    }
    let price = high.add_to(low)?.div(2);
    let array = price.to_arrow(0);
    let mut price = match array.as_any().downcast_ref::<Float64Array>() {
        Some(float_array) => {
            let values: &[f64] = float_array.values();
            let vec_values: Vec<f64> = values.to_vec();
            vec_values
        }
        None => return Err("Failed to downcast to Float64Array".into()),
    };
    let mut smooth = vec![0.0; high.len()];
    let mut detrender = vec![0.0; high.len()];
    // In-phase output signal
    let mut i1 = vec![0.0; high.len()];
    // Quadrature phase output signal
    let mut q1 = vec![0.0; high.len()];


    for bar_index in 0..price.len() {
        if bar_index > 5 {
            smooth[bar_index] = (4.0 * price[bar_index] + 3.0 * price[bar_index - 1] + 2.0 * price[bar_index - 2] + price[bar_index] - 3.0) / 10.0;
            detrender[bar_index] = (0.0962 * smooth[bar_index] + 0.5769 * smooth[bar_index - 2] - 0.5769 * smooth[bar_index - 4] - 0.0962 * smooth[bar_index - 6]) * (0.075 * price[bar_index - 1] + 0.54);
            q1[bar_index] = (0.0962 * detrender[bar_index] + 0.5769 * detrender[bar_index - 2] - 0.5769 * detrender[bar_index - 4] - 0.0962 * detrender[bar_index - 6]) * (0.075 * price[bar_index - 1] + 0.54);
            i1[bar_index] = detrender[bar_index - 3];
        }
    }
    Ok((Series::new("data", q1),Series::new("data", i1)))
}


// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_b_bands()  -> Result<(), Box<dyn std::error::Error>> {
        // let mut rng = rand::thread_rng();
        // let close: Vec<f64> = (0..1000).map(|_| rng.gen_range(1.0..2000.0)).collect();
        let high = Vec::from([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 1.0, 0.0, 11.0, 12.0, 13.0,
        ]);
        let low = Vec::from([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 1.0, 0.0, 11.0, 12.0, 13.0,
        ]);
        let (q1, i1) = ht_trend_line(&Series::new("data", high), &Series::new("data", low))?;
        eprintln!("{:?}", q1);
        eprintln!("{:?}", i1);
        Ok(())
    }
}
