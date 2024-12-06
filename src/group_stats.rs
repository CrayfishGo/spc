use crate::statistics::Statistics;
use crate::{is_alternating, is_decreasing, is_increasing, Rounding, RoundingContext, SpcRule, SpcRuleValidationResult};

const A2: [f64; 26] = [
    0.0, 0.0, 1.880, 1.023, 0.729, 0.577, 0.483, 0.419, 0.373, 0.337, 0.308, 0.285, 0.266, 0.249,
    0.235, 0.223, 0.212, 0.203, 0.194, 0.187, 0.180, 0.173, 0.167, 0.162, 0.157, 0.153,
];
const d2: [f64; 26] = [
    0.0, 0.0, 1.128, 1.693, 2.059, 2.326, 2.534, 2.704, 2.847, 2.97, 3.078, 3.173, 3.258, 3.336,
    3.407, 3.472, 3.532, 3.588, 3.64, 3.689, 3.735, 3.778, 3.819, 3.858, 3.895, 3.931,
];

const D3: [f64; 26] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.076, 0.136, 0.184, 0.223, 0.256, 0.283, 0.307, 0.328,
    0.347, 0.363, 0.378, 0.391, 0.403, 0.415, 0.425, 0.434, 0.443, 0.451, 0.459,
];
const D4: [f64; 26] = [
    0.0, 0.0, 3.267, 2.571, 2.282, 2.114, 2.004, 1.924, 1.864, 1.816, 1.777, 1.744, 1.717, 1.693,
    1.672, 1.653, 1.637, 1.622, 1.608, 1.597, 1.585, 1.575, 1.566, 1.557, 1.548, 1.541,
];

const A3: [f64; 26] = [
    0.0, 0.0, 2.659, 1.954, 1.628, 1.427, 1.287, 1.182, 1.099, 1.032, 0.975, 0.927, 0.886, 0.850,
    0.817, 0.789, 0.763, 0.739, 0.718, 0.698, 0.680, 0.663, 0.647, 0.633, 0.619, 0.606,
];
const c4: [f64; 26] = [
    0.0, 0.0, 0.7979, 0.8862, 0.9213, 0.94, 0.9515, 0.9594, 0.965, 0.9693, 0.9727, 0.9754, 0.9776,
    0.9794, 0.981, 0.9823, 0.9835, 0.9845, 0.9854, 0.9862, 0.9869, 0.9876, 0.9882, 0.9887, 0.9892,
    0.9896,
];

const B3: [f64; 26] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.030, 0.118, 0.185, 0.239, 0.284, 0.321, 0.354, 0.382, 0.406,
    0.428, 0.448, 0.446, 0.482, 0.497, 0.510, 0.523, 0.534, 0.545, 0.555, 0.565,
];
const B4: [f64; 26] = [
    0.0, 0.0, 3.276, 2.568, 2.266, 2.089, 1.970, 1.882, 1.815, 1.761, 1.716, 1.679, 1.640, 1.618,
    1.594, 1.572, 1.552, 1.534, 1.518, 1.503, 1.490, 1.477, 1.466, 1.455, 1.445, 1.435,
];

#[derive(Debug, Eq, PartialEq)]
pub enum GroupStatsChartType {
    RChart,
    XbarRChart,
    SChart,
    XbarSChart,
}

#[derive(Debug)]
pub struct GroupStats {
    cl: f64,
    ucl: f64,
    lcl: f64,
    pub chart_type: GroupStatsChartType,
    data: Vec<Vec<f64>>,
    sub_group_size: usize,
    all_data: Vec<f64>,
    ranges: Vec<f64>,
    stddev: Vec<f64>,
    average: Vec<f64>,
    range_average: f64,
    range_stddev: f64,
    stddev_average: f64,
    stddev_stddev: f64,
    average_average: f64,
    average_stddev: f64,
    all_average: f64,
    all_stddev: f64,
    sigma_estimate: f64,
    minimum: Vec<f64>,
    maximum: Vec<f64>,
    dirty: bool,
    group_count: usize,
    rounding_ctx: Option<RoundingContext>,
}

impl GroupStats {
    pub fn apply_rule_validation(&mut self, rules: Vec<SpcRule>) -> Vec<SpcRuleValidationResult> {
        let mut res = vec![];
        let chart_data = self.chart_data();
        let chart_average = self.chart_average();
        let sigma = self.chart_sigma();
        for rule in rules {
            let mut bad_point_index = vec![];
            let mut passed = true;
            match rule {
                SpcRule::Rule1Beyond3Sigma(p, s) => {
                    let mut ucl = chart_average + s as f64 * sigma;
                    let mut lcl = chart_average - s as f64 * sigma;
                    if let Some(ctx) = &self.rounding_ctx {
                        ucl = ucl.scale(ctx.scale, &ctx.rounding_mode);
                        lcl = lcl.scale(ctx.scale, &ctx.rounding_mode);
                    }
                    for (index, &value) in chart_data.iter().enumerate() {
                        if value > ucl || value < lcl {
                            bad_point_index.push(index);
                        }
                    }
                    if bad_point_index.len() >= p {
                        passed = false;
                    }
                    if passed {
                        bad_point_index.clear();
                    }
                }

                SpcRule::Rule2Of3Beyond2Sigma(p, n, s) | SpcRule::Rule4Of5Beyond1Sigma(p, n, s) => {
                    if chart_data.len() >= n {
                        let mut ucl = chart_average + s as f64 * sigma;
                        let mut lcl = chart_average - s as f64 * sigma;
                        if let Some(ctx) = &self.rounding_ctx {
                            ucl = ucl.scale(ctx.scale, &ctx.rounding_mode);
                            lcl = lcl.scale(ctx.scale, &ctx.rounding_mode);
                        }
                        for i in 0..chart_data.len().saturating_sub(n - 1) {
                            let window = &chart_data[i..i + n]; // Take n consecutive elements
                            let count = window.iter().filter(|&&x| x > ucl || x < lcl).count();
                            if count >= p {
                                passed = false;
                                for (offset, &value) in window.iter().enumerate() {
                                    if value > ucl || value < lcl {
                                        if !bad_point_index.contains(&(offset + i)) {
                                            bad_point_index.push(i + offset);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                SpcRule::Rule6PointsUpOrDown(p) => {
                    if chart_data.len() >= p {
                        for i in 0..chart_data.len().saturating_sub(p - 1) {
                            let window = &chart_data[i..i + p];
                            if is_increasing(window) || is_decreasing(window) {
                                passed = false;
                                for j in 0..window.len() {
                                    if !bad_point_index.contains(&(i + j)) {
                                        bad_point_index.push(i + j);
                                    }
                                }
                            }
                        }
                    }
                }
                SpcRule::Rule8PointsAboveOrBelowCenter(p) => {
                    let mut ucl = chart_average + sigma;
                    let mut lcl = chart_average - sigma;
                    if let Some(ctx) = &self.rounding_ctx {
                        ucl = ucl.scale(ctx.scale, &ctx.rounding_mode);
                        lcl = lcl.scale(ctx.scale, &ctx.rounding_mode);
                    }
                    if chart_data.len() >= p {
                        for i in 0..chart_data.len().saturating_sub(p - 1) {
                            let window = &chart_data[i..i + p];
                            let count = window.iter().filter(|&&x| x > ucl || x < lcl).count();
                            if count >= p {
                                passed = false;
                                for (offset, &value) in window.iter().enumerate() {
                                    if value < ucl || value > lcl {
                                        if !bad_point_index.contains(&(i + offset)) {
                                            bad_point_index.push(i + offset);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                SpcRule::Rule9PointsOnSameSideOfCenter(p) => {
                    if chart_data.len() >= p {
                        for i in 0..chart_data.len().saturating_sub(p - 1) {
                            let window = &chart_data[i..i + p];
                            let flag = window
                                .iter()
                                .all(|&x| x < chart_average || x > chart_average);
                            if flag {
                                passed = false;
                                for (offset, &value) in window.iter().enumerate() {
                                    if !bad_point_index.contains(&(i + offset)) {
                                        bad_point_index.push(i + offset);
                                    }
                                }
                            }
                        }
                    }
                }
                SpcRule::Rule14PointsOscillating(p) => {
                    if chart_data.len() >= p {
                        for i in 0..chart_data.len().saturating_sub(p - 1) {
                            let window = &chart_data[i..i + p];
                            if is_alternating(window) {
                                passed = false;
                                for j in 0..window.len() {
                                    if !bad_point_index.contains(&(i + j)) {
                                        bad_point_index.push(i + j);
                                    }
                                }
                            }
                        }
                    }
                }
                SpcRule::Rule15PointsWithin1Sigma(p, s) => {
                    if chart_data.len() >= p {
                        let mut ucl = chart_average + s as f64 * sigma;
                        let mut lcl = chart_average - s as f64 * sigma;
                        if let Some(ctx) = &self.rounding_ctx {
                            ucl = ucl.scale(ctx.scale, &ctx.rounding_mode);
                            lcl = lcl.scale(ctx.scale, &ctx.rounding_mode);
                        }
                        for i in 0..chart_data.len().saturating_sub(p - 1) {
                            let window = &chart_data[i..i + p];
                            let count = window.iter().filter(|&&x| x < ucl || x > lcl).count();
                            if count >= p {
                                passed = false;
                                for (offset, &value) in window.iter().enumerate() {
                                    if value < ucl || value > lcl {
                                        if !bad_point_index.contains(&(i + offset)) {
                                            bad_point_index.push(i + offset);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            let mut bad_point_data = vec![];
            for i in 0..chart_data.len() {
                if bad_point_index.contains(&i) {
                    bad_point_data.push(chart_data[i]);
                }
            }
            res.push(SpcRuleValidationResult {
                rule,
                bad_point_index,
                bad_point_data,
                validation_passed: passed,
            });
        }
        res
    }
}


impl GroupStats {
    pub fn new(
        sub_group_size: usize,
        chart_type: GroupStatsChartType,
    ) -> Result<GroupStats, String> {
        if sub_group_size < 2 || sub_group_size > 25 {
            return Err("GroupStats: sub_group_size must be in range 2..25".to_string());
        }
        Ok(Self {
            cl: 0.0,
            ucl: 0.0,
            lcl: 0.0,
            chart_type,
            data: vec![],
            sub_group_size,
            all_data: vec![],
            ranges: vec![],
            stddev: vec![],
            average: vec![],
            range_average: 0.0,
            range_stddev: 0.0,
            stddev_average: 0.0,
            stddev_stddev: 0.0,
            average_average: 0.0,
            average_stddev: 0.0,
            all_average: 0.0,
            all_stddev: 0.0,
            sigma_estimate: 0.0,
            minimum: vec![],
            maximum: vec![],
            dirty: true,
            group_count: 100,
            rounding_ctx: None,
        })
    }

    pub fn add_data(&mut self, group_data: &[f64]) -> Result<(), String> {
        if group_data.len() != self.sub_group_size {
            return Err(format!(
                "GroupStats: Trying to add groupData with size {} not equal to sub_group_size {}",
                group_data.len(),
                self.sub_group_size
            ));
        }
        self.data.push(group_data.to_vec());
        self.all_data.extend_from_slice(group_data);
        let mut range = group_data.range();
        let mut stddev = group_data.std_dev();
        let mut average = group_data.average();
        let mut minimum = group_data.min();
        let mut maximum = group_data.max();
        if let Some(ctx) = &self.rounding_ctx {
            range = range.scale(ctx.scale, &ctx.rounding_mode);
            stddev = stddev.scale(ctx.scale, &ctx.rounding_mode);
            average = average.scale(ctx.scale, &ctx.rounding_mode);
            minimum = minimum.scale(ctx.scale, &ctx.rounding_mode);
            maximum = maximum.scale(ctx.scale, &ctx.rounding_mode);
        }
        self.ranges.push(range);
        self.stddev.push(stddev);
        self.average.push(average);
        self.minimum.push(minimum);
        self.maximum.push(maximum);
        self.dirty = true;
        if self.data.len() > self.group_count {
            self.data.remove(0);
            self.ranges.remove(0);
            self.stddev.remove(0);
            self.average.remove(0);
            self.minimum.remove(0);
            self.maximum.remove(0);
            if self.sub_group_size > 0 {
                self.all_data.drain(0..self.sub_group_size);
            }
        }
        Ok(())
    }

    pub fn update(&mut self) {
        if !self.dirty {
            return;
        }
        self.range_average = self.ranges.average();
        self.range_stddev = self.ranges.std_dev();
        self.stddev_average = self.stddev.average();
        self.stddev_stddev = self.stddev.std_dev();
        self.average_average = self.average.average();
        self.average_stddev = self.average.std_dev();
        self.all_average = self.all_data.average();
        self.all_stddev = self.all_data.std_dev();

        match self.chart_type {
            GroupStatsChartType::RChart => {
                self.cl = self.range_average;
                self.ucl = D4[self.sub_group_size] * self.range_average;
                self.lcl = D3[self.sub_group_size] * self.range_average;
                self.sigma_estimate = self.range_average / d2[self.sub_group_size];
            }
            GroupStatsChartType::XbarRChart => {
                self.cl = self.average_average;
                self.ucl = self.average_average + A2[self.sub_group_size] * self.range_average;
                self.lcl = self.average_average - A2[self.sub_group_size] * self.range_average;
                self.sigma_estimate = self.range_average / d2[self.sub_group_size];
            }
            GroupStatsChartType::SChart => {
                self.cl = self.stddev_average;
                self.ucl = B4[self.sub_group_size] * self.stddev_average;
                self.lcl = B3[self.sub_group_size] * self.stddev_average;
                self.sigma_estimate = self.stddev_average / c4[self.sub_group_size];
            }
            GroupStatsChartType::XbarSChart => {
                self.cl = self.average_average;
                self.ucl = self.average_average + A3[self.sub_group_size] * self.stddev_average;
                self.lcl = self.average_average - A3[self.sub_group_size] * self.stddev_average;
                self.sigma_estimate = self.stddev_average / c4[self.sub_group_size];
            }
        }
        match &self.rounding_ctx {
            None => {}
            Some(ctx) => {
                self.range_average = self.range_average.scale(ctx.scale, &ctx.rounding_mode);
                self.range_stddev = self.range_stddev.scale(ctx.scale, &ctx.rounding_mode);
                self.stddev_average = self.stddev_average.scale(ctx.scale, &ctx.rounding_mode);
                self.stddev_stddev = self.stddev_stddev.scale(ctx.scale, &ctx.rounding_mode);
                self.average_average = self.average_average.scale(ctx.scale, &ctx.rounding_mode);
                self.average_stddev = self.average_stddev.scale(ctx.scale, &ctx.rounding_mode);
                self.all_average = self.all_average.scale(ctx.scale, &ctx.rounding_mode);
                self.all_stddev = self.all_stddev.scale(ctx.scale, &ctx.rounding_mode);

                self.cl = self.cl.scale(ctx.scale, &ctx.rounding_mode);
                self.ucl = self.ucl.scale(ctx.scale, &ctx.rounding_mode);
                self.lcl = self.lcl.scale(ctx.scale, &ctx.rounding_mode);
                self.sigma_estimate = self.sigma_estimate.scale(ctx.scale, &ctx.rounding_mode);
            }
        }
        self.dirty = false;
    }

    pub fn lcl(&self) -> f64 {
        self.lcl
    }

    pub fn ucl(&self) -> f64 {
        self.ucl
    }

    pub fn cl(&self) -> f64 {
        self.cl
    }

    pub fn data(&self) -> Vec<Vec<f64>> {
        self.data.to_vec()
    }

    pub fn chart_data(&mut self) -> Vec<f64> {
        match self.chart_type {
            GroupStatsChartType::RChart => self.ranges.to_vec(),
            GroupStatsChartType::XbarRChart => self.average.to_vec(),
            GroupStatsChartType::SChart => self.stddev.to_vec(),
            GroupStatsChartType::XbarSChart => self.average.to_vec(),
        }
    }

    pub fn chart_average(&mut self) -> f64 {
        match self.chart_type {
            GroupStatsChartType::RChart => self.range_average,
            GroupStatsChartType::XbarRChart => self.average_average,
            GroupStatsChartType::SChart => self.stddev_average,
            GroupStatsChartType::XbarSChart => self.average_average,
        }
    }

    pub fn chart_sigma(&mut self) -> f64 {
        (self.ucl - self.chart_average()) / 3.0
    }

    pub fn sub_group_size(&self) -> usize {
        self.sub_group_size
    }

    pub fn ranges(&self) -> Vec<f64> {
        self.ranges.to_vec()
    }

    pub fn stddev(&self) -> Vec<f64> {
        self.stddev.to_vec()
    }

    pub fn average(&self) -> Vec<f64> {
        self.average.to_vec()
    }

    pub fn range_average(&self) -> f64 {
        self.range_average
    }

    pub fn range_stddev(&self) -> f64 {
        self.range_stddev
    }

    pub fn stddev_average(&self) -> f64 {
        self.stddev_average
    }

    pub fn stddev_stddev(&self) -> f64 {
        self.stddev_stddev
    }

    pub fn average_average(&self) -> f64 {
        self.average_average
    }

    pub fn average_stddev(&self) -> f64 {
        self.average_stddev
    }

    pub fn all_average(&self) -> f64 {
        self.all_average
    }

    pub fn all_stddev(&self) -> f64 {
        self.all_stddev
    }

    pub fn sigma_estimate(&self) -> f64 {
        self.sigma_estimate
    }

    pub fn minimum(&self) -> Vec<f64> {
        self.minimum.to_vec()
    }

    pub fn maximum(&self) -> Vec<f64> {
        self.maximum.to_vec()
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }

    pub fn rounding_ctx(&self) -> &Option<RoundingContext> {
        &self.rounding_ctx
    }

    pub fn set_rounding_ctx(&mut self, rounding_ctx: Option<RoundingContext>) {
        self.rounding_ctx = rounding_ctx;
    }

    pub fn set_group_count(&mut self, group_count: usize) {
        self.group_count = group_count;
    }
}

#[cfg(test)]
mod test_group_stats {
    use crate::group_stats::{GroupStats, GroupStatsChartType};
    use crate::RoundingMode::RoundHalfUp;
    use crate::{RoundingContext, SpcRule};

    #[test]
    pub fn test_xbar_r_chart() {
        let v1 = vec![
            0.65, 0.75, 0.75, 0.60, 0.70, 0.60, 0.75, 0.60, 0.65, 0.60, 0.80, 0.85, 0.70, 0.65,
            0.90, 0.75, 0.75, 0.75, 0.65, 0.60, 0.50, 0.60, 0.80, 0.65, 0.65,
        ];
        let v2 = vec![
            0.70, 0.85, 0.80, 0.70, 0.75, 0.75, 0.80, 0.70, 0.80, 0.70, 0.75, 0.75, 0.70, 0.70,
            0.80, 0.80, 0.70, 0.70, 0.65, 0.60, 0.55, 0.80, 0.65, 0.60, 0.70,
        ];
        let v3 = vec![
            0.65, 0.75, 0.80, 0.70, 0.65, 0.75, 0.65, 0.80, 0.85, 0.60, 0.90, 0.85, 0.75, 0.85,
            0.80, 0.75, 0.85, 0.60, 0.85, 0.65, 0.65, 0.65, 0.75, 0.65, 0.70,
        ];
        let v4 = vec![
            0.65, 0.85, 0.70, 0.75, 0.85, 0.85, 0.75, 0.75, 0.85, 0.80, 0.50, 0.65, 0.75, 0.75,
            0.75, 0.80, 0.70, 0.70, 0.65, 0.60, 0.80, 0.65, 0.65, 0.60, 0.60,
        ];
        let v5 = vec![
            0.85, 0.65, 0.75, 0.65, 0.80, 0.70, 0.70, 0.75, 0.75, 0.65, 0.80, 0.70, 0.70, 0.60,
            0.85, 0.65, 0.80, 0.60, 0.70, 0.65, 0.80, 0.75, 0.65, 0.70, 0.65,
        ];

        let mut xbar_r_chart_stats = GroupStats::new(5, GroupStatsChartType::XbarRChart).unwrap();
        xbar_r_chart_stats.set_group_count(100);
        xbar_r_chart_stats.set_rounding_ctx(Some(RoundingContext::new(2, RoundHalfUp)));
        for i in 0..v1.len() {
            let _r = xbar_r_chart_stats
                .add_data(&vec![v1[i], v2[i], v3[i], v4[i], v5[i]])
                .unwrap();
        }

        xbar_r_chart_stats.update();
        let ucl = xbar_r_chart_stats.ucl();
        let lcl = xbar_r_chart_stats.lcl();
        let cl = xbar_r_chart_stats.cl();
        let average = xbar_r_chart_stats.average();
        let ranges = xbar_r_chart_stats.ranges();
        println!("ucl: {}", ucl);
        println!("cl: {}", cl);
        println!("lcl: {}", lcl);
        println!("average: {:?}", average);
        println!("range: {:?}", ranges);
        let res = xbar_r_chart_stats.apply_rule_validation(vec![
            SpcRule::Rule1Beyond3Sigma(1, 2),
            SpcRule::Rule2Of3Beyond2Sigma(2, 3, 1),
        ]);
        println!("res: {:#?}", res);
    }
}
