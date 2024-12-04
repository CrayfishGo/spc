use crate::statistics::Statistics;

const A2: [f64; 11] = [
    0.0, 0.0, 1.880, 1.187, 0.796, 0.691, 0.548, 0.508, 0.433, 0.412, 0.362,
];
const E2: [f64; 11] = [
    0.0, 0.0, 2.660, 1.772, 1.457, 1.290, 1.184, 1.109, 1.054, 1.010, 0.975,
];
const d2: [f64; 11] = [
    0.0, 0.0, 1.128, 1.693, 2.059, 2.326, 2.534, 2.704, 2.847, 2.97, 3.078,
];
const D3: [f64; 11] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.076, 0.136, 0.184, 0.223,
];
const D4: [f64; 11] = [
    0.0, 0.0, 3.267, 2.574, 2.282, 2.114, 2.004, 1.924, 1.864, 1.816, 1.777,
];

#[derive(Debug)]
pub enum MovingStatsChartType {
    IndividualsChart,
    MovingAverageChart,
    MovingRangeChart,
}

#[derive(Debug)]
pub struct MovingStats {
    cl: f64,
    ucl: f64,
    lcl: f64,
    pub chart_type: MovingStatsChartType,
    data: Vec<f64>,
    range_data: Vec<f64>,
    sub_group_size: usize,
    range_span_size: usize,
    all_data: Vec<f64>,
    range: f64,
    stddev: f64,
    average: f64,
    minimum: f64,
    maximum: f64,
    median: f64,
    dirty: bool,
    sigma_estimate: f64,
    max_elements: usize,
    ucl_data: Vec<f64>,
    lcl_data: Vec<f64>,
}

impl MovingStats {
    pub fn new(
        sub_group_size: usize,
        chart_type: MovingStatsChartType,
        range_span_size: Option<usize>,
    ) -> Result<Self, String> {
        if sub_group_size < 2 || sub_group_size > 10 {
            return Err("MovingStats: sub_group_size must be in range 2..10".to_string());
        }

        Ok(Self {
            cl: 0.0,
            ucl: 0.0,
            lcl: 0.0,
            chart_type,
            data: vec![],
            range_data: vec![],
            sub_group_size,
            range_span_size: range_span_size.unwrap_or(2),
            all_data: vec![],
            range: 0.0,
            stddev: 0.0,
            average: 0.0,
            minimum: 0.0,
            maximum: 0.0,
            median: 0.0,
            dirty: false,
            sigma_estimate: 0.0,
            max_elements: 100,
            ucl_data: vec![],
            lcl_data: vec![],
        })
    }

    pub fn add_data(&mut self, value: f64) {
        self.data.push(value);
        if self.data.len() > self.max_elements {
            self.data.remove(0);
        }
    }

    pub fn update(&mut self) {
        if !self.dirty {
            return;
        }

        match self.chart_type {
            MovingStatsChartType::IndividualsChart => {
                self.average = self.data.average();
                self.minimum = self.data.min();
                self.maximum = self.data.max();
                self.stddev = self.data.std_dev();
                self.range = self.data.range();
                if !self.data.is_empty() {
                    self.median = self.data.median();
                }
                self.range_data.clear();
                let mut vec = vec![];
                for i in 0..self.data.len() {
                    vec.clear();
                    if i < self.range_span_size - 1 {
                        self.range_data.push(f64::NAN);
                    } else {
                        for j in 0..self.range_span_size {
                            let index = i - j;
                            vec.push(self.data[index]);
                        }
                        self.range_data.push(vec.range());
                    }
                }
                let range_average = self.range_data.average();
                self.sigma_estimate = range_average / d2[self.range_span_size];
                let ucl = self.average + E2[self.range_span_size] * range_average;
                let lcl = self.average - E2[self.range_span_size] * range_average;
                for _ in 0..self.range_data.len() {
                    self.ucl_data.push(ucl);
                    self.lcl_data.push(lcl);
                }
            }

            MovingStatsChartType::MovingAverageChart => {
                // todo
            }
            MovingStatsChartType::MovingRangeChart => {
                self.range_data.clear();
                let mut vec = vec![];
                for i in 0..self.data.len() {
                    vec.clear();
                    if i < self.range_span_size - 1 {
                        self.range_data.push(f64::NAN);
                    } else {
                        for j in 0..self.range_span_size {
                            let index = i - j;
                            vec.push(self.data[index]);
                        }
                        self.range_data.push(vec.range());
                    }
                }
                self.average = self.range_data.average();
                self.minimum = self.range_data.min();
                self.maximum = self.range_data.max();
                self.stddev = self.range_data.std_dev();
                self.range = self.range_data.range();
                if !self.range_data.is_empty() {
                    self.median = self.range_data.median();
                }
                self.sigma_estimate = self.average / d2[self.range_span_size];
                let ucl = D4[self.range_span_size] * self.average;
                let lcl = D3[self.range_span_size] * self.average;
                for _ in 0..self.range_data.len() {
                    self.ucl_data.push(ucl);
                    self.lcl_data.push(lcl);
                }
            }
        }
        self.dirty = true;
    }
}
