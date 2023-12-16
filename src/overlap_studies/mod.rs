/*
 * @Author: uyplayer
 * @Date: 2023/10/22 12:14
 * @Email: uyplayer@qq.com
 * @File: overlap_studies
 * @Software: RustRover
 * @Dir: rusty-talib / src/overlap_studies
 * @Project_Name: rusty-talib
 * @Description:
 */

// | Function             | Description                                 | Status  |
// |----------------------|---------------------------------------------|---------|
// | BBANDS               | Bollinger Bands                             | Done    |
// | DEMA                 | Double Exponential Moving Average           | Done    |
// | EMA                  | Exponential Moving Average                  | Done    |
// | HT_TRENDLINE         | Hilbert Transform - Instantaneous Trendline | Done    |
// | KAMA                 | Kaufman Adaptive Moving Average             | Done    |
// | MA                   | Moving Average                              | Done    |
// | MAMA                 | MESA Adaptive Moving Average                | Done    |
// | MAVP                 | Moving average with variable period         | Pending |
// | MIDPOINT             | MidPoint over period                        | Pending |
// | MIDPRICE             | Midpoint Price over period                  | Pending |
// | SAR                  | Parabolic SAR                               | Pending |
// | SAREXT               | Parabolic SAR - Extended                    | Pending |
// | SMA                  | Simple Moving Average                       | Pending |
// | T3                   | Triple Exponential Moving Average (T3)      | Pending |
// | TEMA                 | Triple Exponential Moving                   | Pending |

/// This module contains various moving average functions and indicators.
mod bbands;
mod exponential_moving_average;
mod moving_average;
mod simple_moving_average;
mod double_exponential_moving_average;
mod ht_trend_line;
mod kama;
mod mama;
mod mavp;


pub use bbands::b_bands;


pub use exponential_moving_average::exponential_moving_average;


pub use moving_average::moving_average;


pub use simple_moving_average::simple_moving_average;


pub use double_exponential_moving_average::double_exponential_moving_average;


pub use ht_trend_line::ht_trend_line;


pub use kama::kama;

pub use mama::mama;

pub use mavp::mavp;














