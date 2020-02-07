pub struct MovingAverage {
    data: Vec<f64>,
    data_sum: f64,
    data_avg: f64,
    max_length: usize,
}

impl MovingAverage {
    pub fn new(max_length: usize) -> Self {
        Self {
            data: vec![],
            data_sum: 0f64,
            data_avg: 0f64,
            max_length,
        }
    }

    pub fn insert(&mut self, data_entry: f64) -> f64 {
        self.data.push(data_entry);
        self.data_sum += data_entry;

        if self.data.len() > self.max_length {
            let popped = self.data.remove(0);
            self.data_sum -= popped;
        }

        self.data_avg = self.data_sum / self.data.len() as f64;

        if self.data.len() < (self.max_length / 8) {
            data_entry
        } else {
            self.data_avg
        }
    }

    pub fn newest_entry(&self) -> f64 {
        *self.data.last().unwrap_or(&0f64)
    }

    pub fn oldest_entry(&self) -> f64 {
        *self.data.first().unwrap_or(&0f64)
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
