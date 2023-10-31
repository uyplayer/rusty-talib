# Rusty-talib

Rusty-talib is a technical analysis library written in pure Rust, providing various methods for calculating technical indicators.

## Features

- Provides various technical analysis tools including overlap studies, momentum indicators, volume indicators, and more.
- Allows enabling or disabling specific feature modules as needed.

## Usage

To use Rusty-talib, you can add it as a dependency and select specific feature modules to enable as needed. For example:

```toml
[dependencies]
rusty-talib = { version = "0.1.0", features = ["overlap_studies"] }
```

### Feature Modules
#### Rusty-talib includes the following feature modules:
- [X] Overlap Studies
- [ ] Momentum Indicators
- [ ] Volume Indicators
- [ ] Volatility Indicators
- [ ] Price Transform
- [ ] Cycle Indicators
- [ ] Pattern Recognition
- [ ] Statistical Functions
- [ ] Math Transform
- [ ] Math Operators


##### Overlap Studies

| Function             | Description                                 | Status  |
|----------------------|---------------------------------------------|---------|
| BBANDS               | Bollinger Bands                             | Done    |
| DEMA                 | Double Exponential Moving Average           | Done    |
| EMA                  | Exponential Moving Average                  | Done    |
| HT_TRENDLINE         | Hilbert Transform - Instantaneous Trendline | Done    |
| KAMA                 | Kaufman Adaptive Moving Average             | Pending |
| MA                   | Moving Average                              | Done    | 
| MAMA                 | MESA Adaptive Moving Average                | Pending |
| MAVP                 | Moving average with variable period         | Pending |
| MIDPOINT             | MidPoint over period                        | Pending |
| MIDPRICE             | Midpoint Price over period                  | Pending |
| SAR                  | Parabolic SAR                               | Pending |
| SAREXT               | Parabolic SAR - Extended                    | Pending |
| SMA                  | Simple Moving Average                       | Done    |
| T3                   | Triple Exponential Moving Average (T3)      | Pending |
| TEMA                 | Triple Exponential Moving                   | Pending | 


##### Momentum Indicators

| Function | Description                                            | Status  |
|----------|--------------------------------------------------------|---------|
| ADX      | Average Directional Movement Index                     | Pending |
| ADXR     | Average Directional Movement Index Rating              | Pending |
| APO      | Absolute Price Oscillator                              | Pending |
| AROON    | Aroon                                                  | Pending |
| AROONOSC | Aroon Oscillator                                       | Pending |
| BOP      | Balance Of Power                                       | Pending |
| CCI      | Commodity Channel Index                                | Pending |
| CMO      | Chande Momentum Oscillator                             | Pending |
| DX       | Directional Movement Index                             | Pending |
| MACD     | Moving Average Convergence/Divergence                  | Pending |
| MACDEXT  | MACD with controllable MA type                         | Pending |
| MACDFIX  | Moving Average Convergence/Divergence Fix 12/26        | Pending |
| MFI      | Money Flow Index                                       | Pending |
| MINUS_DI | Minus Directional Indicator                            | Pending |
| MINUS_DM | Minus Directional Movement                             | Pending |
| MOM      | Momentum                                               | Pending |
| PLUS_DI  | Plus Directional Indicator                             | Pending |
| PLUS_DM  | Plus Directional Movement                              | Pending |
| PPO      | Percentage Price Oscillator                            | Pending |
| ROC      | Rate of change : ((price/prevPrice)-1)*100             | Pending |
| ROCP     | Rate of change Percentage: (price-prevPrice)/prevPrice | Pending |
| ROCR     | Rate of change ratio: (price/prevPrice)                | Pending |
| ROCR100  | Rate of change ratio 100 scale: (price/prevPrice)*100  | Pending |
| RSI      | Relative Strength Index                                | Pending |
| STOCH    | Stochastic                                             | Pending |
| STOCHF   | Stochastic Fast                                        | Pending |
| STOCHRSI | Stochastic Relative Strength Index                     | Pending |
| TRIX     | 1-day Rate-Of-Change (ROC) of a Triple Smooth EMA      | Pending |
| ULTOSC   | Ultimate Oscillator                                    | Pending |
| WILLR    | Williams' %R                                           | Pending |


##### Volume Indicators

| Function | Description            | Status  |
|----------|------------------------|---------|
| AD       | Chaikin A/D Line       | Pending |
| ADOSC    | Chaikin A/D Oscillator | Pending |
| OBV      | On Balance Volume      | Pending |


##### Volatility Indicators

| Function | Description                   | Status  |
|----------|-------------------------------|---------|
| ATR      | Average True Range            | Pending |
| NATR     | Normalized Average True Range | Pending |
| TRANGE   | True Range                    | Pending |


##### Price Transform

| Function  | Description                | Status  |
|-----------|----------------------------|---------|
| AVGPRICE  | Average Price              | Pending |
| MEDPRICE  | Median Price               | Pending |
| TYPPRICE  | Typical Price              | Pending |
| WCLPRICE  | Weighted Close Price       | Pending |


##### Cycle Indicators

| Function     | Description                               | Status  |
|--------------|-------------------------------------------|---------|
| HT_DCPERIOD  | Hilbert Transform - Dominant Cycle Period | Pending |
| HT_DCPHASE   | Hilbert Transform - Dominant Cycle Phase  | Pending |
| HT_PHASOR    | Hilbert Transform - Phasor Components     | Pending |
| HT_SINE      | Hilbert Transform - SineWave              | Pending |
| HT_TRENDMODE | Hilbert Transform - Trend vs Cycle Mode   | Pending |


##### Pattern Recognition

| Function            | Description                                           | Status  |
|---------------------|-------------------------------------------------------|---------|
| CDL2CROWS           | Two Crows                                             | Pending |
| CDL3BLACKCROWS      | Three Black Crows                                     | Pending |
| CDL3INSIDE          | Three Inside Up/Down                                  | Pending |
| CDL3LINESTRIKE      | Three-Line Strike                                     | Pending |
| CDL3OUTSIDE         | Three Outside Up/Down                                 | Pending |
| CDL3STARSINSOUTH    | Three Stars In The South                              | Pending |
| CDL3WHITESOLDIERS   | Three Advancing White Soldiers                        | Pending |
| CDLABANDONEDBABY    | Abandoned Baby                                        | Pending |
| CDLADVANCEBLOCK     | Advance Block                                         | Pending |
| CDLBELTHOLD         | Belt-hold                                             | Pending |
| CDLBREAKAWAY        | Breakaway                                             | Pending |
| CDLCLOSINGMARUBOZU  | Closing Marubozu                                      | Pending |
| CDLCONCEALBABYSWALL | Concealing Baby Swallow                               | Pending |
| CDLCOUNTERATTACK    | Counterattack                                         | Pending |
| CDLDARKCLOUDCOVER   | Dark Cloud Cover                                      | Pending |
| CDLDOJI             | Doji                                                  | Pending |
| CDLDOJISTAR         | Doji Star                                             | Pending |
| CDLDRAGONFLYDOJI    | Dragonfly Doji                                        | Pending |
| CDLENGULFING        | Engulfing Pattern                                     | Pending |
| CDLEVENINGDOJISTAR  | Evening Doji Star                                     | Pending |
| CDLEVENINGSTAR      | Evening Star                                          | Pending |
| CDLGAPSIDESIDEWHITE | Up/Down-gap side-by-side white lines                  | Pending |
| CDLGRAVESTONEDOJI   | Gravestone Doji                                       | Pending |
| CDLHAMMER           | Hammer                                                | Pending |
| CDLHANGINGMAN       | Hanging Man                                           | Pending |
| CDLHARAMI           | Harami Pattern                                        | Pending |
| CDLHARAMICROSS      | Harami Cross Pattern                                  | Pending |
| CDLHIGHWAVE         | High-Wave Candle                                      | Pending |
| CDLHIKKAKE          | Hikkake Pattern                                       | Pending |
| CDLHIKKAKEMOD       | Modified Hikkake Pattern                              | Pending |
| CDLHOMINGPIGEON     | Homing Pigeon                                         | Pending |
| CDLIDENTICAL3CROWS  | Identical Three Crows                                 | Pending |
| CDLINNECK           | In-Neck Pattern                                       | Pending |
| CDLINVERTEDHAMMER   | Inverted Hammer                                       | Pending |
| CDLKICKING          | Kicking                                               | Pending |
| CDLKICKINGBYLENGTH  | Kicking - bull/bear determined by the longer marubozu | Pending |
| CDLLADDERBOTTOM     | Ladder Bottom                                         | Pending |
| CDLLONGLEGGEDDOJI   | Long Legged Doji                                      | Pending |
| CDLLONGLINE         | Long Line Candle                                      | Pending |
| CDLMARUBOZU         | Marubozu                                              | Pending |
| CDLMATCHINGLOW      | Matching Low                                          | Pending |
| CDLMATHOLD          | Mat Hold                                              | Pending |
| CDLMORNINGDOJISTAR  | Morning Doji Star                                     | Pending |
| CDLMORNINGSTAR      | Morning Star                                          | Pending |
| CDLONNECK           | On-Neck Pattern                                       | Pending |
| CDLPIERCING         | Piercing Pattern                                      | Pending |
| CDLRICKSHAWMAN      | Rickshaw Man                                          | Pending |
| CDLRISEFALL3METHODS | Rising/Falling Three Methods                          | Pending |
| CDLSEPARATINGLINES  | Separating Lines                                      | Pending |
| CDLSHOOTINGSTAR     | Shooting Star                                         | Pending |
| CDLSHORTLINE        | Short Line Candle                                     | Pending |
| CDLSPINNINGTOP      | Spinning Top                                          | Pending |
| CDLSTALLEDPATTERN   | Stalled Pattern                                       | Pending |
| CDLSTICKSANDWICH    | Stick Sandwich                                        | Pending |
| CDLTAKURI           | Takuri (Dragonfly Doji with very long lower shadow)   | Pending |
| CDLTASUKIGAP        | Tasuki Gap                                            | Pending |
| CDLTHRUSTING        | Thrusting Pattern                                     | Pending |
| CDLTRISTAR          | Tristar Pattern                                       | Pending |
| CDLUNIQUE3RIVER     | Unique 3 River                                        | Pending |
| CDLUPSIDEGAP2CROWS  | Upside Gap Two Crows                                  | Pending |
| CDLXSIDEGAP3METHODS | Upside/Downside Gap Three Methods                     | Pending |


##### Statistic Functions

| Function            | Description                           | Status  |
|---------------------|---------------------------------------|---------|
| BETA                | Beta                                  | Pending |
| CORREL              | Pearson's Correlation Coefficient (r) | Pending |
| LINEARREG           | Linear Regression                     | Pending |
| LINEARREG_ANGLE     | Linear Regression Angle               | Pending |
| LINEARREG_INTERCEPT | Linear Regression Intercept           | Pending |
| LINEARREG_SLOPE     | Linear Regression Slope               | Pending |
| STDDEV              | Standard Deviation                    | Pending |
| TSF                 | Time Series Forecast                  | Pending |
| VAR                 | Variance                              | Pending |


##### Math Transform Functions

| Function                | Description                      | Status   |
|-------------------------|----------------------------------|----------|
| ACOS                    | Vector Trigonometric ACos        | Pending  |
| ASIN                    | Vector Trigonometric ASin        | Pending  |
| ATAN                    | Vector Trigonometric ATan        | Pending  |
| CEIL                    | Vector Ceil                      | Pending  |
| COS                     | Vector Trigonometric Cos         | Pending  |
| COSH                    | Vector Trigonometric Cosh        | Pending  |
| EXP                     | Vector Arithmetic Exp            | Pending  |
| FLOOR                   | Vector Floor                     | Pending  |
| LN                      | Vector Log Natural               | Pending  |
| LOG10                   | Vector Log10                     | Pending  |
| SIN                     | Vector Trigonometric Sin         | Pending  |
| SINH                    | Vector Trigonometric Sinh        | Pending  |
| SQRT                    | Vector Square Root               | Pending  |
| TAN                     | Vector Trigonometric Tan         | Pending  |
| TANH                    | Vector Trigonometric Tanh        | Pending  |


##### Math Operator Functions

| Function    | Description                                                      | Status  |
|-------------|------------------------------------------------------------------|---------|
| ADD         | Vector Arithmetic Add                                            | Pending |
| DIV         | Vector Arithmetic Div                                            | Pending |
| MAX         | Highest value over a specified period                            | Pending |
| MAXINDEX    | Index of the highest value over a specified period               | Pending |
| MIN         | Lowest value over a specified period                             | Pending |
| MININDEX    | Index of the lowest value over a specified period                | Pending |
| MINMAX      | Lowest and highest values over a specified period                | Pending |
| MINMAXINDEX | Indexes of the lowest and highest values over a specified period | Pending |
| MULT        | Vector Arithmetic Mult                                           | Pending |
| SUB         | Vector Arithmetic Subtraction                                    | Pending |
| SUM         | Summation                                                        | Pending |


### Example
```rust
use rusty_talib::{self,ErrorMsg};
use polars::prelude::*;

fn main(){
    let random_data: [i32; 7] = [23, 25, 12, 28, 33, 31, 35];
    let close = Series::new("data",random_data);
    let ma = rusty_talib::moving_average(&close,Some(3));
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
    

}

```

```rust
// import polars lib 
use polars::prelude::*;
// Import the rusty-talib library in your Rust project
use rusty_talib;

let random_data: [i32; 7] = [23, 25, 12, 28, 33, 31, 35];
let close = Series::new("data",random_data);
// Use a specific function from the overlap studies feature module
let result = rusty_talib::moving_average(close,2);
// Series type 
eprintln!("{:?}",result);
```
### Casting Series to Vec< f64>

```rust
// import Float64Array
use polars::export::arrow::array::{Float64Array};

let array = result.to_arrow(0);

let vec_values = match array.as_any().downcast_ref::<Float64Array>() {
    Some(float_array) => {
        let values: &[f64] = float_array.values();
        let vec_values: Vec<f64> = values.to_vec();
        vec_values
    }
    None => return Err("Failed to downcast to Float64Array".into()),
};
eprintln!("{:?}",vec_values);

```
### Casting Series to single data
```rust
// pull single data from Series type result
for i in 0..result.len() {
    let elem :f64= src.get(i)?.try_extract::<f64>()?;
    eprintln!("{:?}",elem);
}
```


### License
#### This project is licensed under the MIT License.
Feel free to customize the content further as needed.