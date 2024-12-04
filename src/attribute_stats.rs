use num_traits::Float;

#[derive(Debug)]
pub enum AttributeStatsChartType {
    PChart,
    NpChart,
    CChart,
    UChart,
}

#[derive(Debug)]
pub struct AttributeStats {
    cl: f64,
    ucl: f64,
    lcl: f64,
    chart_type: AttributeStatsChartType,
    max_elements: usize,
    samples: Vec<f64>,
    defects: Vec<f64>,
    data: Vec<f64>,
    average: f64,
    dirty: bool,
}

impl AttributeStats {
    pub fn new(max_elements: Option<usize>, chart_type: AttributeStatsChartType) -> AttributeStats {
        Self {
            cl: 0.0,
            ucl: 0.0,
            lcl: 0.0,
            chart_type,
            max_elements: max_elements.unwrap_or(100),
            samples: vec![],
            defects: vec![],
            data: vec![],
            average: 0.0,
            dirty: false,
        }
    }

    pub fn update(&mut self, sigma_multiple: Option<f64>) {
        if !self.dirty {
            return;
        }
        self.data.clear();
        self.ucl = 0.0;
        self.lcl = 0.0;
        self.cl = 0.0;
        if self.defects.is_empty() {
            return;
        }

        let sigma_m = sigma_multiple.unwrap_or(3.0);

        match self.chart_type {
            AttributeStatsChartType::PChart => {
                let mut total1 = 0.0;
                let mut total2 = 0.0;
                for i in 0..self.defects.len() {
                    total1 += self.defects[i];
                    total2 += self.samples[i];
                    self.data.push(self.defects[i] / self.samples[i]);
                }
                self.average = total1 / total2;
                let n_avg = total2 / self.samples.len() as f64;
                self.ucl =
                    self.average + sigma_m * ((self.average * (1.0 - self.average)).sqrt() / n_avg);
                self.lcl =
                    self.average - sigma_m * ((self.average * (1.0 - self.average)).sqrt() / n_avg);
                self.lcl = self.lcl.max(0.0);
                self.cl = self.average;
            }
            AttributeStatsChartType::NpChart => {
                let mut sum = 0.0;
                for d in self.defects {
                    sum += d;
                    self.data.push(d);
                }
                let n = self.defects.len() as f64;
                let k = self.samples.get(0).unwrap();
                let pbar = sum / (n * k);
                self.average = sum / n;
                self.ucl = self.average + sigma_m * (self.average * (1.0 - pbar)).sqrt();
                self.lcl = self.average - sigma_m * (self.average * (1.0 - pbar)).sqrt();
                self.lcl = self.lcl.max(0.0);
                self.cl = self.average;
            }
            AttributeStatsChartType::CChart => {
                let mut sum = 0.0;
                for d in self.defects {
                    sum += d;
                    self.data.push(d);
                }
                let n = self.defects.len() as f64;
                self.average = sum / n;
                let sigma = self.average.sqrt();
                self.ucl = self.average + sigma_m * sigma;
                self.lcl = self.average - sigma_m * sigma;
                self.lcl = self.lcl.max(0.0);
                self.cl = self.average;
            }
            AttributeStatsChartType::UChart => {
                let mut csum = 0.0;
                let mut nsum = 0.0;
                for i in 0..self.defects.len() {
                    csum += self.defects[i];
                    nsum += self.samples[i];
                    self.data.push(self.defects[i] / self.samples[i]);
                }
                self.average = csum / nsum;
                let n_avg = nsum / self.samples.len() as f64;
                self.ucl = self.average + sigma_m * (self.average / n_avg).sqrt();
                self.lcl = self.average - sigma_m * (self.average / n_avg).sqrt();
                self.lcl = self.lcl.max(0.0);
                self.cl = self.average;
            }
        }
        self.dirty = true;
    }

    pub fn add_data(&mut self, defect: f64, sample: f64) -> Result<(), String> {
        if self.chart_type.eq(&AttributeStatsChartType::NpChart) {
            if !self.samples.is_empty() {
                let f = self.samples.get(0).unwrap();
                if f != sample {
                    return Err("Can't change number test for NP charts");
                }
            }
        }
        self.defects.push(defect);
        self.samples.push(sample);
        loop {
            if self.defects.len() <= self.max_elements {
                break;
            }
            self.defects.remove(0);
            self.samples.remove(0);
        }
        self.dirty = true;
    }

    pub fn lcl(&mut self, sigma_multiple: Option<f64>) -> f64 {
        self.update(sigma_multiple);
        self.lcl
    }

    pub fn ucl(&mut self, sigma_multiple: Option<f64>) -> f64 {
        self.update(sigma_multiple);
        self.ucl
    }

    pub fn cl(&mut self, sigma_multiple: Option<f64>) -> f64 {
        self.update(sigma_multiple);
        self.cl
    }

    pub fn chart_type(&self) -> &AttributeStatsChartType {
        &self.chart_type
    }

    pub fn max_elements(&self) -> usize {
        self.max_elements
    }

    pub fn samples(&self) -> &Vec<f64> {
        &self.samples
    }

    pub fn defects(&self) -> &Vec<f64> {
        &self.defects
    }

    pub fn data(&mut self) -> &Vec<f64> {
        self.update(None);
        &self.data
    }

    pub fn average(&mut self) -> f64 {
        self.update(None);
        self.average
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }
}
