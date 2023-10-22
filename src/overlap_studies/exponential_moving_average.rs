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

    for i in 1..src.len() {
        let elem  = src.get(i)?.try_extract::<f64>()?;
        eprintln!("{:?}",elem);
    }

    let result_series = Series::new("data",[1,2,3,3,10]);
    Ok(result_series)
}


// unit test
#[cfg(test)]
mod tests {
    use rand::Rng;
    use super::*;

    #[test]
    fn test_b_bands() ->Result<(), Box<dyn std::error::Error>>{
        let random_data: [i32; 7] = [23, 25, 12, 28, 33, 31, 35];
        let close = Series::new("data",random_data);
        let res = exponential_moving_average(&close,Some(3))?;
        eprintln!("{:?}",res);
        Ok(())
    }
}