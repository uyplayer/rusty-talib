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


use crate::ErrorMsg;
use polars::prelude::*;

pub fn exponential_moving_average<'a>(src:&'a Series, time_period:u32)-> Result<Series, Box<dyn std::error::Error>>{

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

    let alpha = 2.0 / (time_period + 1) as f64;
    let mut ema: Option<f64> = None;

    let mut result = Vec::with_capacity(src.len());

    for v in src.f64()? {
        match ema {
            Some(e) => ema = Some(v? * alpha? + e * (1.0 - alpha)),
            None => ema = Some(v.unwrap()),
        }
        result.push(ema.unwrap());
    }
    Ok(Series::new("data", result))

}