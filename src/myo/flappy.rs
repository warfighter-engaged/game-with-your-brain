use super::emg_filters;

const SAMPLE_RATE: emg_filters::SampleFrequency = emg_filters::SampleFrequency::Freq1000Hz;
const NOTCH_FREQ: emg_filters::NotchFrequency = emg_filters::NotchFrequency::Freq60Hz;

struct Flappy {
    threshold: i32,
    start_emitting: bool,
    emit_number: u32,

    filter: emg_filters::EMGFilters,

    // for get_emg_count
    integral_data: i32,
    integral_data_eve: i32,
    remain_flag: bool,
    time_millis: u32,
    time_begin_zero: u32,
    fist_num: i32,
}

impl Flappy {
    pub fn new() -> Self {
        let filter = emg_filters::EMGFilters::new(SAMPLE_RATE, NOTCH_FREQ, true, true, true);
        Flappy {
            threshold: 0, // 0 in the calibration process
            start_emitting: false,
            emit_number: 0,
            filter,

            integral_data: 0,
            integral_data_eve: 0,
            remain_flag: false,
            time_millis: 0,
            time_begin_zero: 0,
            fist_num: 0,
        }
    }

    /// Gets the sEMG envelope; use this value directly to determine the threshold during calibration
    pub fn get_envelope(&mut self, data: u16) -> i32 {
        let data_after_filter = self.filter.update(data as i32); // filter processing
        data_after_filter.pow(2) // get envelope by squaring the input
    }

    /// Takes in the result of an analog read at 1000Hz. Returns true if
    /// the muscle is flexed, false otherwise.
    pub fn update(&mut self, data: u16) -> bool {
        let envelope = self.get_envelope(data);
        let envelope = if envelope > self.threshold {
            envelope
        } else {
            0
        }; // // The data set below the base value is set to 0, indicating that it is in a relaxed state

        let result = self.start_emitting;
        if result {
            self.emit_number += 1;
        }

        self.get_emg_count(envelope);

        result
    }

    /// If get EMG signal, return true
    fn get_emg_count(&mut self, gforce_envelope: i32) -> bool {
        const TIME_STANDARD: u32 = 200;

        /*
        The integral is processed to continuously add the signal value
        and compare the integral value of the previous sampling to determine whether the signal is continuous
        */
        self.integral_data_eve = self.integral_data;
        self.integral_data += gforce_envelope;

        /*
        If the integral is constant, and it doesn't equal 0, then the time is recorded;
        If the value of the integral starts to change again, the remainflag is true, and the time record will be re-entered next time
        */
        if self.integral_data_eve == 0 && gforce_envelope > 0 {
            // We've only seen 0's. We now see a value above 0
            self.start_emitting = true;
        }

        if (self.integral_data_eve == self.integral_data) && (self.integral_data_eve != 0) {
            self.time_millis += 1;
            if self.remain_flag {
                self.time_begin_zero = self.time_millis;
                self.remain_flag = false;
                return false;
            }
            // If the integral value exceeds 200 ms, the integral value is clear 0,return that get EMG signal
            if (self.time_millis - self.time_begin_zero) > TIME_STANDARD {
                self.integral_data_eve = 0;
                self.integral_data = 0;
                self.start_emitting = false;
                return true;
            }
            false
        } else {
            self.remain_flag = true;
            false
        }
    }
}
