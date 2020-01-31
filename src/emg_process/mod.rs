mod moving_avg;
mod peak_to_peak;

pub use moving_avg::*;
pub use peak_to_peak::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EmgOptions {
    ReferenceAvailable,
    ReferenceUnavailable,
    HighPassFilterOn,
    HighPassFilterOff,
}

pub struct EMG {
    data: MovingAverage,

    remove_low_frequency: EmgOptions,
    for_hpf: Option<MovingAverage>,

    reference_available: EmgOptions,
    lpf_data: MovingAverage,
    lpf_reference: Option<MovingAverage>,
}

impl EMG {
    pub fn new(
        sample_frequency: usize,
        range: f64,
        min_emg_frequency: usize,
        max_emg_frequency: usize,
        remove_low_frequency: EmgOptions,
        reference_available: EmgOptions,
    ) -> Self {
        let data = MovingAverage::new((sample_frequency as f64 * range) as usize);
        let for_hpf = if remove_low_frequency == EmgOptions::HighPassFilterOn {
            Some(MovingAverage::new(sample_frequency * 2 / min_emg_frequency))
        } else {
            None
        };

        let length = 0.125f64 * sample_frequency as f64 / max_emg_frequency as f64;
        let length = if length >= 1f64 { length as usize } else { 1 };
        let lpf_data = MovingAverage::new(length);
        let lpf_reference = if reference_available == EmgOptions::ReferenceAvailable {
            Some(MovingAverage::new(length))
        } else {
            None
        };

        Self {
            data,

            remove_low_frequency,
            for_hpf,

            reference_available,
            lpf_data,
            lpf_reference,
        }
    }

    pub fn filter_emg(&mut self, data: f64) -> f64 {
        let data = if self.reference_available != EmgOptions::ReferenceAvailable {
            self.lpf_data.insert(data)
        } else {
            data
        };

        if self.remove_low_frequency == EmgOptions::HighPassFilterOn {
            let neutral_value = self.for_hpf.as_mut().unwrap().insert(data);
            self.data.insert((data - neutral_value).abs())
        } else {
            self.data.insert(data.abs())
        }
    }

    pub fn filter_emg_r(&mut self, data: f64, reference_data: f64) -> f64 {
        if self.reference_available != EmgOptions::ReferenceAvailable {
            return -1f64;
        }

        let clean_data = self.lpf_data.insert(data);
        let _clean_reference = self.lpf_reference.as_mut().unwrap().insert(reference_data);
        let data = clean_data - reference_data;
        self.filter_emg(data)
    }
}
