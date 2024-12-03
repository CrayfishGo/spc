use crate::error::StatsError;
use num_traits::Pow;
use num_traits::float::FloatCore;
use num_traits::real::Real;
use std::borrow::Borrow;

/// Enumeration of possible tie-breaking strategies
/// when computing ranks
#[derive(Debug, Copy, Clone)]
pub enum RankTieBreaker {
    /// Replaces ties with their mean
    Average,
    /// Replace ties with their minimum
    Min,
    /// Replace ties with their maximum
    Max,
    /// Permutation with increasing values at each index of ties
    First,
}

/// The `Statistics` trait provides a host of statistical utilities for
/// analyzing
/// data sets
pub trait Statistics {
    /// Returns the minimum value in the data
    ///
    /// # Remarks
    ///
    /// Returns `f64::NAN` if data is empty or an entry is `f64::NAN`
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64;
    /// use spc::statistics::Statistics;
    ///
    /// let x = &[];
    /// assert!(Statistics::min(x).is_nan());
    ///
    /// let y = &[0.0, f64::NAN, 3.0, -2.0];
    /// assert!(Statistics::min(y).is_nan());
    ///
    /// let z = &[0.0, 3.0, -2.0];
    /// assert_eq!(Statistics::min(z), -2.0);
    /// ```
    fn min(&self) -> f64;

    /// Returns the maximum value in the data
    ///
    /// # Remarks
    ///
    /// Returns `f64::NAN` if data is empty or an entry is `f64::NAN`
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64;
    /// use spc::statistics::Statistics;
    ///
    /// let x = &[];
    /// assert!(Statistics::max(x).is_nan());
    ///
    /// let y = &[0.0, f64::NAN, 3.0, -2.0];
    /// assert!(Statistics::max(y).is_nan());
    ///
    /// let z = &[0.0, 3.0, -2.0];
    /// assert_eq!(Statistics::max(z), 3.0);
    /// ```
    fn max(&self) -> f64;

    /// Returns the minimum absolute value in the data
    ///
    /// # Remarks
    ///
    /// Returns `f64::NAN` if data is empty or an entry is `f64::NAN`
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64;
    /// use spc::statistics::Statistics;
    ///
    /// let x = &[];
    /// assert!(x.abs_min().is_nan());
    ///
    /// let y = &[0.0, f64::NAN, 3.0, -2.0];
    /// assert!(y.abs_min().is_nan());
    ///
    /// let z = &[0.0, 3.0, -2.0];
    /// assert_eq!(z.abs_min(), 0.0);
    /// ```
    fn abs_min(&self) -> f64;

    /// Returns the maximum absolute value in the data
    ///
    /// # Remarks
    ///
    /// Returns `f64::NAN` if data is empty or an entry is `f64::NAN`
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64;
    /// use spc::statistics::Statistics;
    ///
    /// let x = &[];
    /// assert!(x.abs_max().is_nan());
    ///
    /// let y = &[0.0, f64::NAN, 3.0, -2.0];
    /// assert!(y.abs_max().is_nan());
    ///
    /// let z = &[0.0, 3.0, -2.0, -8.0];
    /// assert_eq!(z.abs_max(), 8.0);
    /// ```
    fn abs_max(&self) -> f64;

    /// Evaluates the sample mean, an estimate of the population
    /// mean.
    ///
    /// # Remarks
    ///
    /// Returns `f64::NAN` if data is empty or an entry is `f64::NAN`
    ///
    /// # Examples
    ///
    /// ```
    /// #[macro_use]
    /// extern crate spc;
    ///
    /// use std::f64;
    /// use spc::statistics::Statistics;
    ///
    /// # fn main() {
    /// let x = &[];
    /// assert!(x.average().is_nan());
    ///
    /// let y = &[0.0, f64::NAN, 3.0, -2.0];
    /// assert!(y.average().is_nan());
    ///
    /// let z = &[0.0, 3.0, -2.0];
    /// assert_almost_eq!(z.average(), 1.0 / 3.0, 1e-15);
    /// # }
    /// ```
    fn average(&self) -> f64;

    /// Evaluates the geometric mean of the data
    ///
    /// # Remarks
    ///
    /// Returns `f64::NAN` if data is empty or an entry is `f64::NAN`.
    /// Returns `f64::NAN` if an entry is less than `0`. Returns `0`
    /// if no entry is less than `0` but there are entries equal to `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// #[macro_use]
    /// extern crate spc;
    /// use std::f64;
    /// use spc::statistics::Statistics;
    ///
    /// # fn main() {
    ///
    /// let x = &[];
    /// assert!(x.geometric_average().is_nan());
    ///
    /// let y = &[0.0, f64::NAN, 3.0, -2.0];
    /// assert!(y.geometric_average().is_nan());
    ///
    /// let mut z = &[0.0, 3.0, -2.0];
    /// assert!(z.geometric_average().is_nan());
    ///
    /// z = &[0.0, 3.0, 2.0];
    /// assert_eq!(z.geometric_average(), 0.0);
    ///
    /// z = &[1.0, 2.0, 3.0];
    /// // test value from online calculator, could be more accurate
    /// assert_almost_eq!(z.geometric_average(), 1.81712, 1e-5);
    /// # }
    /// ```
    fn geometric_average(&self) -> f64;

    /// Evaluates the harmonic mean of the data
    ///
    /// 计算数据的调和平均值
    ///
    /// # Remarks
    ///
    /// Returns `f64::NAN` if data is empty or an entry is `f64::NAN`, or if
    /// any value
    /// in data is less than `0`. Returns `0` if there are no values less than
    /// `0` but
    /// there exists values equal to `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// #[macro_use]
    /// extern crate spc;
    ///
    /// use std::f64;
    /// use spc::statistics::Statistics;
    ///
    /// # fn main() {
    /// let x = &[];
    /// assert!(x.harmonic_average().is_nan());
    ///
    /// let y = &[0.0, f64::NAN, 3.0, -2.0];
    /// assert!(y.harmonic_average().is_nan());
    ///
    /// let mut z = &[0.0, 3.0, -2.0];
    /// assert!(z.harmonic_average().is_nan());
    ///
    /// z = &[0.0, 3.0, 2.0];
    /// assert_eq!(z.harmonic_average(), 0.0);
    ///
    /// z = &[1.0, 2.0, 3.0];
    /// // test value from online calculator, could be more accurate
    /// assert_almost_eq!(z.harmonic_average(), 1.63636, 1e-5);
    /// # }
    /// ```
    fn harmonic_average(&self) -> f64;

    /// Estimates the unbiased population variance from the provided samples
    ///
    /// # Remarks
    ///
    /// On a dataset of size `N`, `N-1` is used as a normalizer (Bessel's
    /// correction).
    ///
    /// Returns `f64::NAN` if data has less than two entries or if any entry is
    /// `f64::NAN`
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64;
    /// use spc::statistics::Statistics;
    ///
    /// let x = &[];
    /// assert!(x.variance().is_nan());
    ///
    /// let y = &[0.0, f64::NAN, 3.0, -2.0];
    /// assert!(y.variance().is_nan());
    ///
    /// let z = &[0.0, 3.0, -2.0];
    /// assert_eq!(z.variance(), 19.0 / 3.0);
    /// ```
    fn variance(&self) -> f64;

    /// Estimates the unbiased population standard deviation from the provided
    /// samples
    ///
    /// # Remarks
    ///
    /// On a dataset of size `N`, `N-1` is used as a normalizer (Bessel's
    /// correction).
    ///
    /// Returns `f64::NAN` if data has less than two entries or if any entry is
    /// `f64::NAN`
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64;
    /// use spc::statistics::Statistics;
    ///
    /// let x = &[];
    /// assert!(x.std_dev().is_nan());
    ///
    /// let y = &[0.0, f64::NAN, 3.0, -2.0];
    /// assert!(y.std_dev().is_nan());
    ///
    /// let z = &[0.0, 3.0, -2.0];
    /// assert_eq!(z.std_dev(), (19f64 / 3.0).sqrt());
    /// ```
    fn std_dev(&self) -> f64;

    /// Evaluates the population variance from a full population.
    ///
    /// # Remarks
    ///
    /// On a dataset of size `N`, `N` is used as a normalizer and would thus
    /// be biased if applied to a subset
    ///
    /// Returns `f64::NAN` if data is empty or an entry is `f64::NAN`
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64;
    /// use spc::statistics::Statistics;
    ///
    /// let x = &[];
    /// assert!(x.population_variance().is_nan());
    ///
    /// let y = &[0.0, f64::NAN, 3.0, -2.0];
    /// assert!(y.population_variance().is_nan());
    ///
    /// let z = &[0.0, 3.0, -2.0];
    /// assert_eq!(z.population_variance(), 38.0 / 9.0);
    /// ```
    fn population_variance(&self) -> f64;

    /// Evaluates the population standard deviation from a full population.
    ///
    /// # Remarks
    ///
    /// On a dataset of size `N`, `N` is used as a normalizer and would thus
    /// be biased if applied to a subset
    ///
    /// Returns `f64::NAN` if data is empty or an entry is `f64::NAN`
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64;
    /// use spc::statistics::Statistics;
    ///
    /// let x = &[];
    /// assert!(x.population_std_dev().is_nan());
    ///
    /// let y = &[0.0, f64::NAN, 3.0, -2.0];
    /// assert!(y.population_std_dev().is_nan());
    ///
    /// let z = &[0.0, 3.0, -2.0];
    /// assert_eq!(z.population_std_dev(), (38f64 / 9.0).sqrt());
    /// ```
    fn population_std_dev(&self) -> f64;

    /// Estimates the unbiased population covariance between the two provided
    ///
    /// 协方差
    /// samples
    ///
    /// # Remarks
    ///
    /// On a dataset of size `N`, `N-1` is used as a normalizer (Bessel's
    /// correction).
    ///
    /// Returns `f64::NAN` if data has less than two entries or if any entry is
    /// `f64::NAN`
    ///
    /// # Panics
    ///
    /// If the two sample containers do not contain the same number of elements
    ///
    /// # Examples
    ///
    /// ```
    /// #[macro_use]
    /// extern crate spc;
    ///
    /// use std::f64;
    /// use spc::statistics::Statistics;
    ///
    /// # fn main() {
    /// let x = &[];
    /// assert!(x.covariance(&[]).is_nan());
    ///
    /// let y1 = &[0.0, f64::NAN, 3.0, -2.0];
    /// let y2 = &[-5.0, 4.0, 10.0, f64::NAN];
    /// assert!(y1.covariance(y2).is_nan());
    ///
    /// let z1 = &[0.0, 3.0, -2.0];
    /// let z2 = &[-5.0, 4.0, 10.0];
    /// assert_almost_eq!(z1.covariance(z2), -5.5, 1e-14);
    /// # }
    /// ```
    fn covariance(&self, other: &Self) -> f64;

    /// Evaluates the population covariance between the two provider populations
    ///
    /// # Remarks
    ///
    /// On a dataset of size `N`, `N` is used as a normalizer and would thus be
    /// biased if applied to a subset
    ///
    /// Returns `f64::NAN` if data is empty or any entry is `f64::NAN`
    ///
    /// # Panics
    ///
    /// If the two sample containers do not contain the same number of elements
    ///
    /// # Examples
    ///
    /// ```
    /// #[macro_use]
    /// extern crate spc;
    ///
    /// use std::f64;
    /// use spc::statistics::Statistics;
    ///
    /// # fn main() {
    /// let x = &[];
    /// assert!(x.population_covariance(&[]).is_nan());
    ///
    /// let y1 = &[0.0, f64::NAN, 3.0, -2.0];
    /// let y2 = &[-5.0, 4.0, 10.0, f64::NAN];
    /// assert!(y1.population_covariance(y2).is_nan());
    ///
    /// let z1 = &[0.0, 3.0, -2.0];
    /// let z2 = &[-5.0, 4.0, 10.0];
    /// assert_almost_eq!(z1.population_covariance(z2), -11.0 / 3.0, 1e-14);
    /// # }
    /// ```
    fn population_covariance(&self, other: &Self) -> f64;

    /// Estimates the quadratic mean (Root Mean Square) of the data
    ///
    /// # Remarks
    ///
    /// Returns `f64::NAN` if data is empty or any entry is `f64::NAN`
    ///
    /// # Examples
    ///
    /// ```
    /// #[macro_use]
    /// extern crate spc;
    ///
    /// use std::f64;
    /// use spc::statistics::Statistics;
    ///
    /// # fn main() {
    /// let x = &[];
    /// assert!(x.quadratic_average().is_nan());
    ///
    /// let y = &[0.0, f64::NAN, 3.0, -2.0];
    /// assert!(y.quadratic_average().is_nan());
    ///
    /// let z = &[0.0, 3.0, -2.0];
    /// // test value from online calculator, could be more accurate
    /// assert_almost_eq!(z.quadratic_average(), 2.08167, 1e-5);
    /// # }
    /// ```
    fn quadratic_average(&self) -> f64;

    /// Calculates the range
    ///
    fn range(&self) -> f64;

    /// 计算偏斜度
    fn skewness(&self) -> f64;

    /// 计算峰度
    fn kurtosis(&self) -> f64;

    /// 计算坡度
    fn slope(&self, other: &Self) -> f64;
}

impl Statistics for [f64] {
    fn min(&self) -> f64 {
        self.iter().copied().reduce(f64::min).unwrap_or(f64::NAN)
    }

    fn max(&self) -> f64 {
        self.iter().copied().reduce(f64::max).unwrap_or(f64::NAN)
    }

    fn abs_min(&self) -> f64 {
        self.iter()
            .map(|&x| x.abs())
            .reduce(f64::min)
            .unwrap_or(f64::NAN)
    }

    fn abs_max(&self) -> f64 {
        self.iter()
            .map(|&x| x.abs())
            .reduce(f64::max)
            .unwrap_or(f64::NAN)
    }

    fn average(&self) -> f64 {
        let sum: f64 = self.iter().sum();
        if self.is_empty() {
            f64::NAN
        } else {
            sum / self.len() as f64
        }
    }

    fn geometric_average(&self) -> f64 {
        let mut i = 0.0;
        let mut sum = 0.0;
        for x in self {
            i += 1.0;
            sum += x.borrow().ln();
        }
        if i > 0.0 { (sum / i).exp() } else { f64::NAN }
    }

    fn harmonic_average(&self) -> f64 {
        let mut i = 0.0;
        let mut sum = 0.0;
        for x in self {
            i += 1.0;
            let borrow = *x.borrow();
            if borrow < 0f64 {
                return f64::NAN;
            }
            sum += 1.0 / borrow;
        }
        if i > 0.0 { i / sum } else { f64::NAN }
    }

    fn variance(&self) -> f64 {
        if self.len() < 2 {
            0.0
        } else {
            let average = self.average();
            let mut v: f64 = 0.0;
            for s in self {
                let x = *s - average;
                v += x * x;
            }
            // N.B., this is _supposed to be_ len-1, not len. If you
            // change it back to len, you will be calculating a
            // population variance, not a sample variance.
            let denom = (self.len() - 1) as f64;
            v / denom
        }
    }

    fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }

    fn population_variance(&self) -> f64 {
        let mut iter = self.into_iter();
        let mut sum = match iter.next() {
            None => return f64::NAN,
            Some(x) => *x.borrow(),
        };
        let mut i = 1.0;
        let mut variance = 0.0;

        for x in iter {
            i += 1.0;
            let borrow = *x.borrow();
            sum += borrow;
            let diff = i * borrow - sum;
            variance += diff * diff / (i * (i - 1.0));
        }
        variance / i
    }

    fn population_std_dev(&self) -> f64 {
        self.population_variance().sqrt()
    }

    fn covariance(&self, other: &Self) -> f64 {
        let mut n = 0.0;
        let mut mean1 = 0.0;
        let mut mean2 = 0.0;
        let mut comoment = 0.0;

        let mut iter = other.into_iter();
        for x in self {
            let borrow = *x.borrow();
            let borrow2 = match iter.next() {
                None => panic!("{}", StatsError::ContainersMustBeSameLength),
                Some(x) => *x.borrow(),
            };
            let old_mean2 = mean2;
            n += 1.0;
            mean1 += (borrow - mean1) / n;
            mean2 += (borrow2 - mean2) / n;
            comoment += (borrow - mean1) * (borrow2 - old_mean2);
        }
        if iter.next().is_some() {
            panic!("{}", StatsError::ContainersMustBeSameLength);
        }

        if n > 1.0 {
            comoment / (n - 1.0)
        } else {
            f64::NAN
        }
    }

    fn population_covariance(&self, other: &Self) -> f64 {
        let mut n = 0.0;
        let mut mean1 = 0.0;
        let mut mean2 = 0.0;
        let mut comoment = 0.0;

        let mut iter = other.into_iter();
        for x in self {
            let borrow = *x.borrow();
            let borrow2 = match iter.next() {
                None => panic!("{}", StatsError::ContainersMustBeSameLength),
                Some(x) => *x.borrow(),
            };
            let old_mean2 = mean2;
            n += 1.0;
            mean1 += (borrow - mean1) / n;
            mean2 += (borrow2 - mean2) / n;
            comoment += (borrow - mean1) * (borrow2 - old_mean2);
        }
        if iter.next().is_some() {
            panic!("{}", StatsError::ContainersMustBeSameLength)
        }
        if n > 0.0 { comoment / n } else { f64::NAN }
    }

    fn quadratic_average(&self) -> f64 {
        let mut i = 0.0;
        let mut average = 0.0;
        for x in self {
            let borrow = *x.borrow();
            i += 1.0;
            average += (borrow * borrow - average) / i;
        }
        if i > 0.0 { average.sqrt() } else { f64::NAN }
    }

    fn range(&self) -> f64 {
        self.max() - self.min()
    }

    fn skewness(&self) -> f64 {
        let mean = self.average();
        let mut variance = 0.0;
        for &value in self {
            variance += (value - mean).powi(2);
        }
        variance /= self.len() as f64;
        let mut skewness = 0.0;
        for &value in self {
            skewness += (value - mean).powi(3);
        }
        skewness /= self.len() as f64;
        skewness /= variance.powf(1.5);
        skewness
    }

    fn kurtosis(&self) -> f64 {
        let mean = self.average();
        let mut variance = 0.0;
        for &value in self {
            variance += (value - mean).powi(2);
        }
        variance /= self.len() as f64;
        let mut kurtosis = 0.0;
        for &value in self {
            kurtosis += (value - mean).powi(4);
        }
        kurtosis /= self.len() as f64;
        kurtosis /= variance.powi(2);
        kurtosis -= 3.0; // 偏峰度修正
        kurtosis
    }

    fn slope(&self, other: &Self) -> f64 {
        let mut iter_x = self.into_iter();
        let mut iter_y = other.into_iter();

        let len_x = self.len();
        let len_y = other.len();
        if len_x != len_y {
            panic!("number of x and y values are not equal");
        }

        let mut numerator = 0.0; // 分子
        let mut denominator = 0.0; // 分母

        while let (Some(x_val), Some(y_val)) = (iter_x.next(), iter_y.next()) {
            let x_f64 = *x_val.borrow();
            let y_f64 = *y_val.borrow();
            numerator += x_f64 * y_f64;
            denominator += x_f64 * x_f64;
        }

        if denominator != 0.0 {
            numerator / denominator
        } else {
            // 如果分母为 0，计算 y 的平均值
            let y_sum: f64 = iter_y.map(|y_val| *y_val.borrow()).sum();
            y_sum / len_x as f64
        }
    }
}

#[cfg(test)]
mod op_test {
    use crate::statistics::Statistics;

    #[test]
    fn test_op() {
        let nums = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        println!("range =    {:?}", nums.range());
        println!("min =      {:?}", nums.min());
        println!("max =      {:?}", nums.max());
        println!("skewness = {:?}", nums.skewness());
        println!("kurtosis = {:?}", nums.kurtosis())
    }
}
