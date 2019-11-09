pub struct PkPk {
    data: Vec<f64>,
    max_values: Vec<f64>,
    min_values: Vec<f64>,

    cur_length: usize,
    cur_max: f64,
    cur_min: f64,

    min_pk_gap: usize,
    max_pk_gap: usize,
}

pub struct PkPkData {
    pub max: f64,
    pub min: f64,
    pub pkpk: f64,
    pub neutral: f64,
}

impl PkPk {
    pub fn new(sample_frequency: usize, min_frequency: usize, max_frequency: usize) -> Self {
        let cur_max = -100_000f64;
        let cur_min = 100_000f64;
        PkPk {
            data: vec![],
            max_values: vec![cur_max],
            min_values: vec![cur_min],
            cur_length: 0,
            cur_max,
            cur_min,
            min_pk_gap: sample_frequency / max_frequency,
            max_pk_gap: sample_frequency / min_frequency,
        }
    }

    pub fn get_pkpk(&mut self, data_entry: f64) -> PkPkData {
        if self.cur_length > (self.max_pk_gap * 2) {
            let popped = self.data.remove(0);
            if (*self.max_values.first().unwrap() - popped).abs() < std::f64::EPSILON {
                self.max_values.remove(0);
            }
            if (*self.min_values.first().unwrap() - popped).abs() < std::f64::EPSILON {
                self.min_values.remove(0);
            }
            self.cur_length -= 1;

            if self.max_values.is_empty() || self.min_values.is_empty() {
                self.max_values.clear();
                self.min_values.clear();

                // scan for new min and max, and push to their queues
                let (min, max) = get_queue_max_min(&self.data);
                self.max_values.push(max);
                self.min_values.push(min);
            }
        }

        self.data.push(data_entry);
        self.cur_length += 1;

        if data_entry > self.cur_max {
            self.cur_max = data_entry;
            self.max_values.clear();
            self.max_values.push(data_entry);
        } else if (data_entry - self.cur_max).abs() < std::f64::EPSILON {
            // found new non-unique max value
            self.max_values.push(data_entry);
        }

        if data_entry < self.cur_min {
            self.cur_min = data_entry;
            self.min_values.clear();
            self.min_values.push(data_entry);
        } else if (data_entry - self.cur_min).abs() < std::f64::EPSILON {
            // found new non-unique min value
            self.min_values.push(data_entry);
        }

        PkPkData {
            max: self.cur_max,
            min: self.cur_min,
            pkpk: self.cur_max - self.cur_min,
            neutral: (self.cur_max - self.cur_min) / 2f64 + self.cur_min
        }
    }
}

fn get_queue_max_min(queue: &[f64]) -> (f64, f64) {
    let mut max = -100_000f64;
    let mut min = 100_000f64;
    for val in queue.iter() {
        if val > &max {
            max = *val;
        }
        if val < &min {
            min = *val;
        }
    }
    (min, max)
}