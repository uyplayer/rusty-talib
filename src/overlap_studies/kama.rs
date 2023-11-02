/*
 * @Author: uyplayer
 * @Date: 2023/11/2 17:18
 * @Email: uyplayer@qq.com
 * @File: kama.rs
 * @Software: RustRover
 * @Dir: rusty-talib / src/overlap_studies
 * @Project_Name: rusty-talib
 * @Description:
 */


//! Formula:
//! - direction = close - close_period
//! - volatility = sumN(abs(close - close_n), period)
//! - effiency_ratio = abs(direction / volatility)
//! - fast = 2 / (fast_period + 1)
//! - slow = 2 / (slow_period + 1)
//!
//! - smfactor = squared(efficienty_ratio * (fast - slow) + slow)
//! - smfactor1 = 1.0  - smfactor
//!
//! - The initial seed value is a Simple Moving Average.
//!
//! See also:
//! - [Kaufman's Adaptive Moving Average (KAMA)](http://fxcodebase.com/wiki/index.php/Kaufman's_Adaptive_Moving_Average_(KAMA))
//! - [MetaTrader 5 AMA Indicator](http://www.metatrader5.com/en/terminal/help/analytics/indicators/trend_indicators/ama)
//! - [CQG Adaptive Moving Average](http://help.cqg.com/cqgic/default.htm#!Documents/adaptivemovingaverag2.htm)
//! - [Python Pandas KAMA Implementation](https://copyprogramming.com/howto/python-pandas-kaufman-adaptive-moving-average-kama)
//! - [StockCharts.com - KAMA](https://school.stockcharts.com/doku.php?id=technical_indicators:kaufman_s_adaptive_moving_average)
//!
use std::ops::{Add, Mul};
use polars::export::arrow::array::Float64Array;
use polars::prelude::*;

/// Calculate Kaufman's Adaptive Moving Average (KAMA).
///
/// # Arguments
///
/// * `src` - A Polars Series containing price data.
/// * `time_period` - The time period for the KAMA calculation (default: 10).
/// * `fast` - The fast period for the KAMA calculation (default: 2).
/// * `slow` - The slow period for the KAMA calculation (default: 30).
///
/// # Returns
///
/// A Polars Series containing KAMA values.
///
/// # Examples
///
/// ```
/// use polars::prelude::*;
/// use rusty_talib::kama;
///
/// let close: [f64; 32] = [
///     35.0, 10.0, 20.0, 56.0, 10.0, 20.0, 56.0, 89.0, 89.0, 76.0, 76.0, 30.0, 10.0, 20.0, 56.0, 89.0, 46.0,
///     10.0, 653.0, 10.0, 20.0, 56.0, 89.0, 30.0, 46.0, 10.0, 653.0, 76.0, 30.0, 46.0, 10.0, 653.0,
/// ];
/// let kama_series = kama(&Series::new("Close", close.iter().copied().collect()), Some(10), Some(2), Some(30)).unwrap();
/// println!("{:?}", kama_series);
/// ```
pub fn kama<'a>(
    src: &'a Series,
    time_period: Option<usize>,
    fast: Option<usize>,
    slow: Option<usize>,
) -> Result<Series, Box<dyn std::error::Error>> {
    let time_period = time_period.unwrap_or(10);
    let fast = fast.unwrap_or(2);
    let slow = slow.unwrap_or(30);

    // Calculate direction and volatility
    let duration = Duration::new(time_period as i64);
    let options = RollingOptionsImpl {
        window_size: duration,
        min_periods: 1,
        ..Default::default()
    };
    let direction = src
        .subtract(&src.shift(time_period as i64))?
        .abs()?
        .fill_null(FillNullStrategy::Backward(None))?;
    let volatility = src
        .subtract(&src.shift(1i64))?
        .abs()?
        .rolling_sum(options)?
        .fill_null(FillNullStrategy::Backward(None))?;

    // Calculate efficiency ratio (ER)
    let er = direction.divide(&volatility)?;

    // Calculate smoothing constants
    let sc_fastest = 2.0 / (fast as f64 + 1.0);
    let sc_slowest = 2.0 / (slow as f64 + 1.0);
    let sc = er.mul(sc_fastest - sc_slowest).add(sc_slowest);
    let sc = sc.multiply(&sc)?;

    // Convert to Vec for calculation
    let sc = sc.to_arrow(0);
    let sc = match sc.as_any().downcast_ref::<Float64Array>() {
        Some(float_array) => {
            let values: &[f64] = float_array.values();
            let vec_values: Vec<f64> = values.to_vec();
            vec_values
        }
        None => return Err("Failed to downcast to Float64Array".into()),
    };

    let src = src.to_arrow(0);
    let src = match src.as_any().downcast_ref::<Float64Array>() {
        Some(float_array) => {
            let values: &[f64] = float_array.values();
            let vec_values: Vec<f64> = values.to_vec();
            vec_values
        }
        None => return Err("Failed to downcast to Float64Array".into()),
    };

    // Calculate KAMA values
    let mut kama_v = vec![0.0; src.len()];
    for i in 1..src.len() {
        let p = src[i];
        let s = sc[i];
        kama_v[i] = kama_v[i - 1] + s * (p - kama_v[i - 1]);
    }

    Ok(Series::new("KAMA", kama_v))
}

// Unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kama() -> Result<(), Box<dyn std::error::Error>> {
        let close: [f64; 32] = [
            35.0, 10.0, 20.0, 56.0, 10.0, 20.0, 56.0, 89.0, 89.0, 76.0, 76.0, 30.0, 10.0, 20.0,
            56.0, 89.0, 46.0, 10.0, 653.0, 10.0, 20.0, 56.0, 89.0, 30.0, 46.0, 10.0, 653.0, 76.0,
            30.0, 46.0, 10.0, 653.0,
        ];
        let kama_series = kama(
            &Series::new("Close", close.iter().copied().collect::<Vec<_>>()),
            Some(10),
            Some(2),
            Some(30),
        )?;
        println!("{:?}", kama_series);
        Ok(())
    }
}
