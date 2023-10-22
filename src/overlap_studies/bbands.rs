/*
 * @Author: uyplayer
 * @Date: 2023/10/22 12:18
 * @Email: uyplayer@qq.com
 * @File: bbands.rs
 * @Software: RustRover
 * @Dir: rusty-talib / src/overlap_studies
 * @Project_Name: rusty-talib
 * @Description:
 */

use polars::prelude::*;
use crate::ErrorMsg;
use crate::overlap_studies::moving_average;

pub fn b_bands<'a>( high: &'a Series,
                    low: &'a Series,
                    close: &'a Series, time_period: Option<u32>, nb_dev_up: Option<u32>, nb_dev_dn: Option<u32>, ma_type: Option<u32>) -> Result<Series, Box<dyn std::error::Error>> {
    let time_period = match time_period {
        Some(p) => p,
        None => 14,
    };
    let nb_dev_up = match nb_dev_up {
        Some(p) => p,
        None => 5,
    };
    let nb_dev_dn = match nb_dev_dn {
        Some(p) => p,
        None => 5,
    };
    let ma_type = match ma_type {
        Some(p) => p,
        None => 0,
    };
    let time_period = time_period as usize ;

    if high.len() < time_period || low.len() < time_period || close.len() < time_period {
        return Err(Box::new(ErrorMsg(
            "src Lengths must be greater than time_period".into(),
        )));
    }

    if high.len() != low.len() || low.len() != close.len() {
        return Err(Box::new(ErrorMsg(
            "Lengths of high, low, and close arrays are not equal".into(),
        )));
    }

    let upper_band = Vec::<f64>::new();
    let middle_band = Vec::<f64>::new();
    let lower_band = Vec::<f64>::new();


    return Err(Box::new(ErrorMsg(
        "Lengths of high, low, and close arrays are not equal".into(),
    )));
}



//
// // unit test
// #[cfg(test)]
// mod tests {
//     use rand::Rng;
//     use super::*;
//
//     #[test]
//     fn test_b_bands() {
//
//         let mut rng = rand::thread_rng();
//         let high: Vec<f64> = (0..1000).map(|_| rng.gen_range(1.0..2000.0)).collect();
//         let low: Vec<f64> = (0..1000).map(|_| rng.gen_range(1.0..2000.0)).collect();
//         let close: Vec<f64> = (0..1000).map(|_| rng.gen_range(1.0..2000.0)).collect();
//         let res = b_bands(&high,&low,&close, Some(10), None, None, None);
//         match res {
//             Ok(_) => eprintln!("ok"),
//             Err(e) => {
//                 if let Some(my_error) = e.downcast_ref::<ErrorMsg>() {
//                     eprintln!("{}", my_error.0);
//                 } else {
//                     eprintln!("An error occurred");
//                 }
//             }
//         }
//
//     }
// }



