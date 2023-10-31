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
// | HT_TRENDLINE         | Hilbert Transform - Instantaneous Trendline | Pending |
// | KAMA                 | Kaufman Adaptive Moving Average             | Pending |
// | MA                   | Moving Average                              | Done    |
// | MAMA                 | MESA Adaptive Moving Average                | Pending |
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

/// Provides functions for calculating Bollinger Bands.
pub use bbands::b_bands;

/// Provides functions for calculating exponential moving averages.
pub use exponential_moving_average::exponential_moving_average;

/// Provides functions for calculating different types of moving averages.
pub use moving_average::moving_average;

/// Provides functions for calculating simple moving averages.
pub use simple_moving_average::simple_moving_average;

/// Provides functions for calculating double exponential moving averages.
pub use double_exponential_moving_average::double_exponential_moving_average;
