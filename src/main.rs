/*
 * @Author: uyplayer
 * @Date: 2023/10/22 14:13
 * @Email: uyplayer@qq.com
 * @File: main.rs
 * @Software: RustRover
 * @Dir: rusty-talib / src
 * @Project_Name: rusty-talib
 * @Description:
 */

use rand;
use rand::Rng;
use rusty_talib;
use rusty_talib::ErrorMsg;
use polars::prelude::*;

fn main()->Result<(), Box<dyn std::error::Error>> {
    let random_data: [i32; 7] = [23, 25, 12, 28, 33, 31, 35];
    let close = Series::new("data",random_data);
    let res = rusty_talib::moving_average(&close,Some(3));
    match res {
        Ok(ma) => {
            assert_eq!(ma.len(),close.len());
            eprintln!("{:?}",ma);
        },
        Err(e) => {
            if let Some(my_error) = e.downcast_ref::<ErrorMsg>() {
                eprintln!("{}", my_error.0);
            } else {
                eprintln!("An error occurred");
            }
        }
    }

    Ok(())

}
