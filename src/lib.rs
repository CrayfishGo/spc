#[macro_use]
extern crate approx;
pub mod attribute_stats;
pub mod error;
pub mod group_stats;
pub mod moving_stats;
pub mod prec;
pub mod statistics;

use crate::statistics::Statistics;
use num_traits::{FromPrimitive, ToPrimitive};
use rust_decimal::{Decimal, RoundingStrategy};
use std::fmt;
use std::fmt::Formatter;

///
/// ```rust
///     use spc_rs::group_stats::{GroupStats, GroupStatsChartType};
///     pub fn main() {
///         use spc_rs::{RoundingContext, SpcRule};
///         use spc_rs::RoundingMode::RoundHalfUp;
///         let v1 = vec![
///             0.65, 0.75, 0.75, 0.60, 0.70, 0.60, 0.75, 0.60, 0.65, 0.60, 0.80, 0.85, 0.70, 0.65,
///             0.90, 0.75, 0.75, 0.75, 0.65, 0.60, 0.50, 0.60, 0.80, 0.65, 0.65,
///         ];
///         let v2 = vec![
///             0.70, 0.85, 0.80, 0.70, 0.75, 0.75, 0.80, 0.70, 0.80, 0.70, 0.75, 0.75, 0.70, 0.70,
///             0.80, 0.80, 0.70, 0.70, 0.65, 0.60, 0.55, 0.80, 0.65, 0.60, 0.70,
///         ];
///         let v3 = vec![
///             0.65, 0.75, 0.80, 0.70, 0.65, 0.75, 0.65, 0.80, 0.85, 0.60, 0.90, 0.85, 0.75, 0.85,
///             0.80, 0.75, 0.85, 0.60, 0.85, 0.65, 0.65, 0.65, 0.75, 0.65, 0.70,
///         ];
///         let v4 = vec![
///             0.65, 0.85, 0.70, 0.75, 0.85, 0.85, 0.75, 0.75, 0.85, 0.80, 0.50, 0.65, 0.75, 0.75,
///             0.75, 0.80, 0.70, 0.70, 0.65, 0.60, 0.80, 0.65, 0.65, 0.60, 0.60,
///         ];
///         let v5 = vec![
///             0.85, 0.65, 0.75, 0.65, 0.80, 0.70, 0.70, 0.75, 0.75, 0.65, 0.80, 0.70, 0.70, 0.60,
///             0.85, 0.65, 0.80, 0.60, 0.70, 0.65, 0.80, 0.75, 0.65, 0.70, 0.65,
///         ];
///
///         let mut xbar_r_chart_stats = GroupStats::new(5, GroupStatsChartType::XbarRChart).unwrap();
///         xbar_r_chart_stats.set_group_count(100);
///         xbar_r_chart_stats.set_rounding_ctx(Some(RoundingContext::new(2, RoundHalfUp)));
///         for i in 0..v1.len() {
///             let _r = xbar_r_chart_stats
///                 .add_data(&vec![v1[i], v2[i], v3[i], v4[i], v5[i]])
///                 .unwrap();
///         }
///
///         let ucl = xbar_r_chart_stats.ucl();
///         let lcl = xbar_r_chart_stats.lcl();
///         let cl = xbar_r_chart_stats.cl();
///         let average = xbar_r_chart_stats.average();
///         let ranges = xbar_r_chart_stats.ranges();
///         assert_eq!(0.82,  ucl);
///         assert_eq!(0.72,  cl);
///         assert_eq!(0.61,  lcl);
///         println!("average: {:?}",average);
///         println!("range: {:?}",ranges);
///
///         // apply spc rules
///         let res = xbar_r_chart_stats.apply_rule_validation(vec![
///             SpcRule::Rule1Beyond3Sigma(1, 3),
///             SpcRule::Rule2Of3Beyond2Sigma(2, 3, 2),
///         ]);
///         println!("res: {:#?}", res);
///
///
///     }
///
/// ```
///
///

#[macro_export]
macro_rules! assert_almost_eq {
    ($a:expr, $b:expr, $prec:expr) => {
        if !$crate::prec::almost_eq($a, $b, $prec) {
            panic!(
                "assertion failed: `abs(left - right) < {:e}`, (left: `{}`, right: `{}`)",
                $prec, $a, $b
            );
        }
    };
}

pub trait Rounding {
    fn scale(&self, scale: u32, rounding_mode: &RoundingMode) -> Self;
}

impl Rounding for f64 {
    fn scale(&self, scale: u32, rounding_mode: &RoundingMode) -> Self {
        let decimal: Decimal = Decimal::from_f64(*self).unwrap();
        match rounding_mode {
            RoundingMode::RoundUp => decimal
                .round_dp_with_strategy(scale, RoundingStrategy::AwayFromZero)
                .to_f64()
                .unwrap(),
            RoundingMode::RoundDown => decimal
                .round_dp_with_strategy(scale, RoundingStrategy::ToZero)
                .to_f64()
                .unwrap(),
            RoundingMode::RoundCeiling => decimal
                .round_dp_with_strategy(scale, RoundingStrategy::ToPositiveInfinity)
                .to_f64()
                .unwrap(),
            RoundingMode::RoundFloor => decimal
                .round_dp_with_strategy(scale, RoundingStrategy::ToNegativeInfinity)
                .to_f64()
                .unwrap(),
            RoundingMode::RoundHalfUp => decimal
                .round_dp_with_strategy(scale, RoundingStrategy::MidpointAwayFromZero)
                .to_f64()
                .unwrap(),
            RoundingMode::RoundHalfDown => decimal
                .round_dp_with_strategy(scale, RoundingStrategy::MidpointTowardZero)
                .to_f64()
                .unwrap(),
            RoundingMode::RoundHalfEven => decimal
                .round_dp_with_strategy(scale, RoundingStrategy::MidpointNearestEven)
                .to_f64()
                .unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct RoundingContext {
    pub scale: u32,
    pub rounding_mode: RoundingMode,
}

impl RoundingContext {
    pub fn new(scale: u32, rounding_mode: RoundingMode) -> Self {
        Self {
            scale,
            rounding_mode,
        }
    }
}

#[derive(Debug)]
pub enum RoundingMode {

    ///
    ///
    ///
    ///          <p>Example:
    ///             <table border>
    ///             <caption><b>Rounding mode UP Examples</b></caption>
    ///          <tr valign=top><th>Input Number</th>
    ///              <th>Input rounded to one digit<br> with {@code UP} rounding
    ///          <tr align=right><td>5.5</td>  <td>6</td>
    ///          <tr align=right><td>2.5</td>  <td>3</td>
    ///          <tr align=right><td>1.6</td>  <td>2</td>
    ///          <tr align=right><td>1.1</td>  <td>2</td>
    ///          <tr align=right><td>1.0</td>  <td>1</td>
    ///          <tr align=right><td>-1.0</td> <td>-1</td>
    ///          <tr align=right><td>-1.1</td> <td>-2</td>
    ///          <tr align=right><td>-1.6</td> <td>-2</td>
    ///          <tr align=right><td>-2.5</td> <td>-3</td>
    ///          <tr align=right><td>-5.5</td> <td>-6</td>
    ///          </table>
    ///
    RoundUp,

    ///
    ///     <p>Example:
    ///          <table border>
    ///           <caption><b>Rounding mode DOWN Examples</b></caption>
    ///          <tr valign=top><th>Input Number</th>
    ///              <th>Input rounded to one digit<br> with {@code DOWN} rounding
    ///          <tr align=right><td>5.5</td>  <td>5</td>
    ///          <tr align=right><td>2.5</td>  <td>2</td>
    ///          <tr align=right><td>1.6</td>  <td>1</td>
    ///          <tr align=right><td>1.1</td>  <td>1</td>
    ///          <tr align=right><td>1.0</td>  <td>1</td>
    ///          <tr align=right><td>-1.0</td> <td>-1</td>
    ///          <tr align=right><td>-1.1</td> <td>-1</td>
    ///          <tr align=right><td>-1.6</td> <td>-1</td>
    ///          <tr align=right><td>-2.5</td> <td>-2</td>
    ///          <tr align=right><td>-5.5</td> <td>-5</td>
    ///          </table>
    ///
    RoundDown,

    ///
    ///     <p>Example:
    ///          <table border>
    ///           <caption><b>Rounding mode CEILING Examples</b></caption>
    ///          <tr valign=top><th>Input Number</th>
    ///              <th>Input rounded to one digit<br> with {@code CEILING} rounding
    ///          <tr align=right><td>5.5</td>  <td>6</td>
    ///          <tr align=right><td>2.5</td>  <td>3</td>
    ///          <tr align=right><td>1.6</td>  <td>2</td>
    ///          <tr align=right><td>1.1</td>  <td>2</td>
    ///          <tr align=right><td>1.0</td>  <td>1</td>
    ///          <tr align=right><td>-1.0</td> <td>-1</td>
    ///          <tr align=right><td>-1.1</td> <td>-1</td>
    ///          <tr align=right><td>-1.6</td> <td>-1</td>
    ///          <tr align=right><td>-2.5</td> <td>-2</td>
    ///          <tr align=right><td>-5.5</td> <td>-5</td>
    ///          </table>
    ///
    RoundCeiling,

    ///
    ///          <p>Example:
    ///          <table border>
    ///           <caption><b>Rounding mode FLOOR Examples</b></caption>
    ///          <tr valign=top><th>Input Number</th>
    ///              <th>Input rounded to one digit<br> with {@code FLOOR} rounding
    ///          <tr align=right><td>5.5</td>  <td>5</td>
    ///          <tr align=right><td>2.5</td>  <td>2</td>
    ///          <tr align=right><td>1.6</td>  <td>1</td>
    ///          <tr align=right><td>1.1</td>  <td>1</td>
    ///          <tr align=right><td>1.0</td>  <td>1</td>
    ///          <tr align=right><td>-1.0</td> <td>-1</td>
    ///          <tr align=right><td>-1.1</td> <td>-2</td>
    ///          <tr align=right><td>-1.6</td> <td>-2</td>
    ///          <tr align=right><td>-2.5</td> <td>-3</td>
    ///          <tr align=right><td>-5.5</td> <td>-6</td>
    ///          </table>
    ///
    RoundFloor,

    ///
    ///     <p>Example:
    ///          <table border>
    ///           <caption><b>Rounding mode HALF_UP Examples</b></caption>
    ///          <tr valign=top><th>Input Number</th>
    ///              <th>Input rounded to one digit<br> with {@code HALF_UP} rounding
    ///          <tr align=right><td>5.5</td>  <td>6</td>
    ///          <tr align=right><td>2.5</td>  <td>3</td>
    ///          <tr align=right><td>1.6</td>  <td>2</td>
    ///          <tr align=right><td>1.1</td>  <td>1</td>
    ///          <tr align=right><td>1.0</td>  <td>1</td>
    ///          <tr align=right><td>-1.0</td> <td>-1</td>
    ///          <tr align=right><td>-1.1</td> <td>-1</td>
    ///          <tr align=right><td>-1.6</td> <td>-2</td>
    ///          <tr align=right><td>-2.5</td> <td>-3</td>
    ///          <tr align=right><td>-5.5</td> <td>-6</td>
    ///          </table>
    ///
    ///
    RoundHalfUp,

    ///
    ///      <p>Example:
    ///          <table border>
    ///           <caption><b>Rounding mode HALF_DOWN Examples</b></caption>
    ///          <tr valign=top><th>Input Number</th>
    ///              <th>Input rounded to one digit<br> with {@code HALF_DOWN} rounding
    ///          <tr align=right><td>5.5</td>  <td>5</td>
    ///          <tr align=right><td>2.5</td>  <td>2</td>
    ///          <tr align=right><td>1.6</td>  <td>2</td>
    ///          <tr align=right><td>1.1</td>  <td>1</td>
    ///          <tr align=right><td>1.0</td>  <td>1</td>
    ///          <tr align=right><td>-1.0</td> <td>-1</td>
    ///          <tr align=right><td>-1.1</td> <td>-1</td>
    ///          <tr align=right><td>-1.6</td> <td>-2</td>
    ///          <tr align=right><td>-2.5</td> <td>-2</td>
    ///          <tr align=right><td>-5.5</td> <td>-5</td>
    ///          </table>
    ///
    ///
    RoundHalfDown,

    /// <p>Example:
    /// <table border>
    ///  <caption><b>Rounding mode HALF_EVEN Examples</b></caption>
    /// <tr valign=top><th>Input Number</th>
    ///     <th>Input rounded to one digit<br> with {@code HALF_EVEN} rounding
    /// <tr align=right><td>5.5</td>  <td>6</td>
    /// <tr align=right><td>2.5</td>  <td>2</td>
    /// <tr align=right><td>1.6</td>  <td>2</td>
    /// <tr align=right><td>1.1</td>  <td>1</td>
    /// <tr align=right><td>1.0</td>  <td>1</td>
    /// <tr align=right><td>-1.0</td> <td>-1</td>
    /// <tr align=right><td>-1.1</td> <td>-1</td>
    /// <tr align=right><td>-1.6</td> <td>-2</td>
    /// <tr align=right><td>-2.5</td> <td>-2</td>
    /// <tr align=right><td>-5.5</td> <td>-6</td>
    /// </table>
    ///
    ///
    ///
    RoundHalfEven,
}

/// Defines mathematical expressions commonly used when computing distribution
/// values as constants

/// Constant value for `sqrt(2 * pi)`
pub const SQRT_2PI: f64 = 2.5066282746310005024157652848110452530069867406099;

/// Constant value for `ln(pi)`
pub const LN_PI: f64 = 1.1447298858494001741434273513530587116472948129153;

/// Constant value for `ln(sqrt(2 * pi))`
pub const LN_SQRT_2PI: f64 = 0.91893853320467274178032973640561763986139747363778;

/// Constant value for `ln(sqrt(2 * pi * e))`
pub const LN_SQRT_2PIE: f64 = 1.4189385332046727417803297364056176398613974736378;

/// Constant value for `ln(2 * sqrt(e / pi))`
pub const LN_2_SQRT_E_OVER_PI: f64 = 0.6207822376352452223455184457816472122518527279025978;

/// Constant value for `2 * sqrt(e / pi)`
pub const TWO_SQRT_E_OVER_PI: f64 = 1.8603827342052657173362492472666631120594218414085755;

/// Constant value for Euler-Masheroni constant `lim(n -> inf) { sum(k=1 -> n)
/// { 1/k - ln(n) } }`
pub const EULER_MASCHERONI: f64 =
    0.5772156649015328606065120900824024310421593359399235988057672348849;

/// Targeted accuracy instantiated over `f64`
pub const ACC: f64 = 10e-11;

#[derive(Debug)]
pub enum SpcRule {
    /// `p` points are beyond from  `s` sigma。
    ///
    /// By default,
    /// * p = 1
    /// * s = 3
    Rule1Beyond3Sigma(usize, usize),

    /// `p` out of `n` consecutive points are beyond from  `s` sigma。
    ///
    /// By default,
    /// * p = 2
    /// * n = 3
    /// * s = 2
    Rule2Of3Beyond2Sigma(usize, usize, usize),

    /// `p` out of `n` consecutive points are beyond from  `s` sigma。
    ///
    /// By default,
    /// * p = 4
    /// * n = 5
    /// * s = 1
    Rule4Of5Beyond1Sigma(usize, usize, usize),
    /// Continuously rising or falling for `n` consecutive points.
    ///
    /// By default,
    /// * n = 6
    Rule6PointsUpOrDown(usize),

    /// `n` consecutive points fall outside the 1σ region but remain within the 2σ region on both sides.
    ///
    /// By default,
    /// * n = 8
    Rule8PointsAboveOrBelowCenter(usize),

    /// `n` consecutive points all fall on the same side of the centerline (either above or below).
    ///
    /// By default,
    /// * n = 9
    Rule9PointsOnSameSideOfCenter(usize),

    /// `n` consecutive points alternate between increasing and decreasing.
    ///
    /// By default,
    /// * n = 14
    Rule14PointsOscillating(usize),

    /// `n` consecutive points all fall within the `Sσ` region on both sides of the centerline.
    ///
    /// By default,
    /// * n = 15
    /// * s = 1
    Rule15PointsWithin1Sigma(usize, usize),
}

impl fmt::Display for SpcRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SpcRule::Rule1Beyond3Sigma(p, s) => {
                write!(f, "{} points beyond from {} sigma", p, s)
            }
            SpcRule::Rule2Of3Beyond2Sigma(p, n, s) => {
                write!(
                    f,
                    "{} out of {} consecutive points beyond from {} sigma",
                    p, n, s
                )
            }
            SpcRule::Rule4Of5Beyond1Sigma(p, n, s) => {
                write!(
                    f,
                    "{} out of {} consecutive points beyond from {} sigma",
                    p, n, s
                )
            }
            SpcRule::Rule6PointsUpOrDown(p) => {
                write!(
                    f,
                    "continuously rising or falling for {}consecutive pointsa",
                    p
                )
            }
            SpcRule::Rule8PointsAboveOrBelowCenter(p) => {
                write!(
                    f,
                    "{} consecutive points fall outside the 1σ region but remain within the 2σ region on both sides",
                    p
                )
            }
            SpcRule::Rule9PointsOnSameSideOfCenter(p) => {
                write!(
                    f,
                    "{} consecutive points all fall on the same side of the centerline (either above or below)",
                    p
                )
            }
            SpcRule::Rule14PointsOscillating(p) => {
                write!(
                    f,
                    "{} consecutive points alternate between increasing and decreasing",
                    p
                )
            }
            SpcRule::Rule15PointsWithin1Sigma(p, s) => {
                write!(
                    f,
                    "{} consecutive points all fall within the {} sigma region on both sides of the centerline",
                    p, s
                )
            }
        }
    }
}

#[derive(Debug)]
pub struct SpcRuleValidationResult {
    pub rule: SpcRule,
    pub bad_point_index: Vec<usize>,
    pub bad_point_data: Vec<f64>,
    pub validation_passed: bool,
}

// 检查是否递增
pub fn is_increasing(data: &[f64]) -> bool {
    data.windows(2).all(|pair| pair[1] > pair[0])
}

// 检查是否递减
pub fn is_decreasing(data: &[f64]) -> bool {
    data.windows(2).all(|pair| pair[1] < pair[0])
}

// 检查是否上下交替趋势
pub fn is_alternating(data: &[f64]) -> bool {
    data.windows(2)
        .zip(data.windows(2).skip(1))
        .all(|(prev, next)| {
            (prev[1] > prev[0] && next[1] < next[0]) || (prev[1] < prev[0] && next[1] > next[0])
        })
}
