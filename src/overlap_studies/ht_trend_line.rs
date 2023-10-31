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


//! calculation for Hilbert Transform - Instantaneous Trendline

use std::ops::{Div, Sub};
use polars::export::arrow::array::Float64Array;
use polars::prelude::*;
use crate::ErrorMsg;


/// https://www.prorealcode.com/prorealtime-indicators/john-ehlers-instantaneous-trendline/
/// https://c.mql5.com/forextsd/forum/59/023inst.pdf
/// https://tw.tradingview.com/script/dFWImthM-blackcat-L2-Ehlers-Hilbert-Transform/
pub fn ht_trend_line<'a>(high: &'a Series, low: &'a Series, time_period: Option<usize>) -> Result<Series, Box<dyn std::error::Error>> {
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
    let mut value3 :Vec<f64> = vec![0.0; high.len()];
    let mut value4: Vec<f64> = vec![0.0; high.len()];
    let mut value5: Vec<f64> = vec![0.0; high.len()];
    let mut value11: Vec<f64> = vec![0.0; high.len()];
    let mut trendline: Vec<f64> = vec![0.0; high.len()];
    let mut in_phase: Vec<f64> = vec![0.0; high.len()];
    let mut quadrature: Vec<f64> = vec![0.0; high.len()];
    let mut phase: Vec<f64> = vec![0.0; high.len()];
    let mut delta_phase: Vec<f64> = vec![0.0; high.len()];
    let mut inst_period: Vec<f64> = vec![0.0; high.len()];

    for bar_index in 0..high.len() {
        if bar_index > 6 {
            value3[bar_index] = price[bar_index] - price[bar_index-7];
            in_phase[bar_index]  = 1.25 * (value3[bar_index-4] - 0.635 * value3[bar_index-2]) + 0.635 * in_phase[bar_index-3];
            quadrature[bar_index]  = value3[bar_index-2] - 0.338 * value3[bar_index]  + 0.338 * quadrature[bar_index-2];


            if (in_phase[bar_index]  + in_phase[bar_index-1]).abs() > 0.0 {
                let a = ((quadrature[bar_index] + quadrature[bar_index-1]) / (in_phase[bar_index] + in_phase[bar_index-1])).abs();
                phase[bar_index]  = a.atan();
            }
            if in_phase[bar_index]  < 0.0 && quadrature[bar_index]  > 0.0 {
                phase[bar_index]  = 180.0 - phase[bar_index] ;
            }
            if in_phase[bar_index]  < 0.0 && quadrature[bar_index]  < 0.0 {
                phase[bar_index]  = 180.0 + phase[bar_index] ;
            }
            if in_phase[bar_index]  > 0.0 && quadrature[bar_index]  < 0.0 {
                phase[bar_index]  = 360.0 - phase[bar_index] ;
            }

            // Compute a differential phase, resolve phase wraparound, and limit delta phase errors
            delta_phase[bar_index] = phase[bar_index-1] - phase[bar_index];
            if phase[bar_index-1] < 90.0 && phase[bar_index] > 270.0 {
                delta_phase[bar_index] = 360.0 + phase[bar_index-1] - phase[bar_index];
            }
            if delta_phase[bar_index] < 7.0 {
                delta_phase[bar_index] = 7.0;
            }
            if delta_phase[bar_index] > 60.0 {
                delta_phase[bar_index] = 60.0;
            }

            for count in 0..41 {
                value4[bar_index] += delta_phase[count];
                if value4[bar_index] > 360.0 && inst_period == 0 {
                    inst_period = count;
                }
            }

            if inst_period[bar_index] == 0 {
                inst_period[bar_index] = inst_period[bar_index-1];
            }
            value5[bar_index] = 0.25 * inst_period[bar_index] as f64 + 0.75 * value5[bar_index-1];
            let period = value5[bar_index].round() as usize;
            let mut sum = 0.0;
            for count in 0..period + 2 {
                sum += price[count];
            }
            if period > 0 {
                trendline[bar_index] = sum / (period + 2) as f64;
            }

            value11[bar_index] = 0.33 * (price[bar_index] + 0.5 * (price[bar_index] - price[bar_index-3])) + 0.67 * value11[1];
        }

        if bar_index < 26 {
            trendline[bar_index] = price[[bar_index]];
            value11[bar_index] = price[[bar_index]];
        }


    }
    Ok((Series::new("data",trendline)))
}



