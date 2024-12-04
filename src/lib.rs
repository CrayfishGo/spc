#[macro_use]
extern crate approx;
pub mod error;
pub mod group_stats;
pub mod prec;
pub mod statistics;
pub mod moving_stats;
pub mod attribute_stats;

use crate::statistics::Statistics;

///
/// ```rust
///     use spc_rs::group_stats::{GroupStats, GroupStatsChartType};
///     pub fn main() {
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
///         let mut xbar_r_chart_stats =
///             GroupStats::new(5, GroupStatsChartType::XbarRChart, None).unwrap();
///         for i in 0..v1.len() {
///             let _r = xbar_r_chart_stats
///                 .add_data(&vec![v1[i], v2[i], v3[i], v4[i], v5[i]])
///                 .unwrap();
///         }
///
///         let ucl = xbar_r_chart_stats.ucl();
///         let lcl = xbar_r_chart_stats.lcl();
///         let cl = xbar_r_chart_stats.cl();
///         let average = xbar_r_chart_stats.average;
///         let ranges = xbar_r_chart_stats.ranges;
///         println!("ucl: {:.2}", ucl);
///         println!("cl:  {:.2}", cl);
///         println!("lcl: {:.2}", lcl);
///         println!(
///             "average: {:?}",
///             average
///                 .into_iter()
///                 .map(|x| (x * 100.0).round() / 100.0)
///                 .collect::<Vec<f64>>()
///         );
///         println!(
///             "range: {:?}",
///             ranges
///                 .into_iter()
///                 .map(|x| (x * 100.0).round() / 100.0)
///                 .collect::<Vec<f64>>()
///         );
///     }
///
/// ```
///
///
///
///
///
///
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
    Rule1Beyond3Sigma(usize, usize),
    Rule2Of3Beyond2Sigma(usize, usize, usize),
    Rule4Of5Beyond1Sigma(usize, usize, usize),
    Rule6PointsUpAndDown(usize),
    Rule8PointsAboveOrBelowCenter(usize),
    Rule9PointsOnSameSideOfCenter(usize),
    Rule14PointsOscillating(usize),
    Rule15PointsWithin1Sigma(usize),
}
