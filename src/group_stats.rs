use crate::SpcChartType;
use crate::statistics::Statistics;

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

const B3: [f64; 24] = [
    0.0, 0.0, 0.0, 0.0, 0.030, 0.118, 0.185, 0.239, 0.284, 0.321, 0.354, 0.382, 0.406, 0.428,
    0.448, 0.446, 0.482, 0.497, 0.510, 0.523, 0.534, 0.545, 0.555, 0.565,
];
const B4: [f64; 26] = [
    0.0, 0.0, 3.276, 2.568, 2.266, 2.089, 1.970, 1.882, 1.815, 1.761, 1.716, 1.679, 1.640, 1.618,
    1.594, 1.572, 1.552, 1.534, 1.518, 1.503, 1.490, 1.477, 1.466, 1.455, 1.445, 1.435,
];

#[derive(Debug)]
pub struct GroupStats {
    pub center_line: f64,
    pub upper_control_limit: f64,
    pub lower_control_limit: f64,
    pub chart_type: SpcChartType,
    pub data: Vec<Vec<f64>>,
    pub sub_group_size: usize,
    pub ranges: Vec<f64>,
    pub stddev: Vec<f64>,
    pub average: Vec<f64>,
    pub range_average: f64,
    pub range_stddev: f64,
    pub stddev_average: f64,
    pub stddev_stddev: f64,
    pub average_average: f64,
    pub average_stddev: f64,
    pub all_average: f64,
    pub all_stddev: f64,
    pub minimum: Vec<f64>,
    pub maximum: Vec<f64>,
    pub dirty: bool,
}

impl GroupStats {
    pub fn new(sub_group_size: usize, chart_type: SpcChartType) -> Result<GroupStats, String> {
        if sub_group_size < 2 || sub_group_size > 25 {
            return Err("GroupStats: sub_group_size must be in range 2..25".to_string());
        }
        Ok(Self {
            center_line: 0.0,
            upper_control_limit: 0.0,
            lower_control_limit: 0.0,
            chart_type,
            data: vec![],
            sub_group_size,
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
            minimum: vec![],
            maximum: vec![],
            dirty: true,
        })
    }

    pub fn add_data(&mut self, group_data: &Vec<f64>) -> Result<(), String> {
        if group_data.len() != self.sub_group_size {
            return Err(format!(
                "GroupStats: Trying to add groupData with size {} not equal to sub_group_size {}",
                group_data.len(),
                self.sub_group_size
            ));
        }
        self.data.push(group_data.to_vec());
        let range = group_data.range();
        let stddev = group_data.std_dev();
        let average = group_data.average();
        let minimum = group_data.min();
        let maximum = group_data.max();
        self.ranges.push(range);
        self.stddev.push(stddev);
        self.average.push(average);
        self.minimum.push(minimum);
        self.maximum.push(maximum);
        self.dirty = true;
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

    }

    pub fn lcl(&mut self) -> f64 {
        self.update();
        self.lower_control_limit
    }

    pub fn ucl(&mut self) -> f64 {
        self.update();
        self.upper_control_limit
    }

    pub fn cl(&mut self) -> f64 {
        self.update();
        self.center_line
    }
}
