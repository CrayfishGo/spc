pub mod error;
pub mod prec;
pub mod statistics;

#[macro_use]
extern crate approx;

use crate::statistics::Statistics;

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
pub struct ControlLimits {
    center_line: f64,
    upper_control_limit: f64,
    lower_control_limit: f64,
}

fn calculate_means(data: &Vec<Vec<f64>>) -> Vec<f64> {
    data.iter().map(|group| group.mean()).collect()
}

/// 计算每组的极差
fn calculate_ranges(data: &Vec<Vec<f64>>) -> Vec<f64> {
    data.iter().map(|group| group.range()).collect()
}
