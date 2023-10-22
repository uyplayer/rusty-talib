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

pub fn exponential_moving_average<'a>(src:&'a Series, time_period:Option<u32>)-> Result<Series, Box<dyn std::error::Error>>{

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


    return Err(Box::new(ErrorMsg(
        "src Length must be greater than time_period".into(),
    )));

}