//! Provides the following filters for processing OYMotion's sEMG signals:
//! 1. an anti-hum notch filter to filter out 50Hz or 60Hz power line noise.
//! 2. a low-pass filter to filter out noises above 150Hz.
//! 3. a high-pass filter to filter out noises below 20Hz.
//! This is based on https://github.com/oymotion/EMGFilters

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum NotchFrequency {
    Freq50Hz = 50,
    Freq60Hz = 60,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SampleFrequency {
    Freq500Hz = 500,
    Freq1000Hz = 1000,
}

// coefficients of transfer function of LPF
// coef[sampleFreqInd][order]
const LPF_NUMERATOR_COEF: [[f32; 3]; 2] = [[0.3913, 0.7827, 0.3913], [0.1311, 0.2622, 0.1311]];
const LPF_DENOMINATOR_COEF: [[f32; 3]; 2] = [[1.0000, 0.3695, 0.1958], [1.0000, -0.7478, 0.2722]];
// coefficients of transfer function of HPF
const HPF_NUMERATOR_COEF: [[f32; 3]; 2] = [[0.8371, -1.6742, 0.8371], [0.9150, -1.8299, 0.9150]];
const HPF_DENOMINATOR_COEF: [[f32; 3]; 2] = [[1.0000, -1.6475, 0.7009], [1.0000, -1.8227, 0.8372]];
// coefficients of transfer function of anti-hum filter
// coef[sampleFreqInd][order] for 50Hz
const AHF_NUMERATOR_COEF_50HZ: [[f32; 6]; 2] = [
    [0.9522, -1.5407, 0.9522, 0.8158, -0.8045, 0.0855],
    [0.5869, -1.1146, 0.5869, 1.0499, -2.0000, 1.0499],
];
const AHF_DENOMINATOR_COEF_50HZ: [[f32; 6]; 2] = [
    [1.0000, -1.5395, 0.9056, 1.0000, -1.1187, 0.3129],
    [1.0000, -1.8844, 0.9893, 1.0000, -1.8991, 0.9892],
];
const AHF_OUTPUT_GAIN_COEF_50HZ: [f32; 2] = [1.3422, 1.4399];
// coef[sampleFreqInd][order] for 60Hz
const AHF_NUMERATOR_COEF_60HZ: [[f32; 6]; 2] = [
    [0.9528, -1.3891, 0.9528, 0.8272, -0.7225, 0.0264],
    [0.5824, -1.0810, 0.5824, 1.0736, -2.0000, 1.0736],
];
const AHF_DENOMINATOR_COEF_60HZ: [[f32; 6]; 2] = [
    [1.0000, -1.3880, 0.9066, 1.0000, -0.9739, 0.2371],
    [1.0000, -1.8407, 0.9894, 1.0000, -1.8584, 0.9891],
];
const AHF_OUTPUT_GAIN_COEF_60HZ: [f32; 2] = [1.3430, 1.4206];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum FilterType {
    LowPass = 0,
    HighPass,
}

struct Filter2nd {
    states: [f32; 2],
    num: [f32; 3],
    den: [f32; 3],
}

impl Filter2nd {
    pub fn new(f_type: FilterType, sample_freq: SampleFrequency) -> Self {
        let states = [0f32; 2];

        let (num, den) = match f_type {
            FilterType::LowPass => {
                // 2nd order butterworth lowpass filter
                // cutoff frequency 150Hz
                match sample_freq {
                    SampleFrequency::Freq500Hz => (LPF_NUMERATOR_COEF[0], LPF_DENOMINATOR_COEF[0]),
                    SampleFrequency::Freq1000Hz => (LPF_NUMERATOR_COEF[1], LPF_DENOMINATOR_COEF[1]),
                }
            }
            FilterType::HighPass => {
                // 2nd order butterworth highpass filter
                // cutoff frequency 20Hz
                match sample_freq {
                    SampleFrequency::Freq500Hz => (HPF_NUMERATOR_COEF[0], HPF_DENOMINATOR_COEF[0]),
                    SampleFrequency::Freq1000Hz => (HPF_NUMERATOR_COEF[1], HPF_DENOMINATOR_COEF[1]),
                }
            }
        };

        Filter2nd { states, num, den }
    }

    pub fn update(&mut self, input: f32) -> f32 {
        let tmp =
            (input - self.den[1] * self.states[0] - self.den[2] * self.states[1]) / self.den[0];
        let output =
            self.num[0] * tmp + self.num[1] * self.states[0] + self.num[2] * self.states[1];
        // save last states
        self.states[1] = self.states[0];
        self.states[0] = tmp;
        output
    }
}

struct Filter4th {
    states: [f32; 4],
    num: [f32; 6],
    den: [f32; 6],
    gain: f32,
}

impl Filter4th {
    pub fn new(sample_freq: SampleFrequency, hum_freq: NotchFrequency) -> Self {
        let states = [0f32; 4];
        let (num, den, gain) = match hum_freq {
            NotchFrequency::Freq50Hz => match sample_freq {
                SampleFrequency::Freq500Hz => (
                    AHF_NUMERATOR_COEF_50HZ[0],
                    AHF_DENOMINATOR_COEF_50HZ[0],
                    AHF_OUTPUT_GAIN_COEF_50HZ[0],
                ),
                SampleFrequency::Freq1000Hz => (
                    AHF_NUMERATOR_COEF_50HZ[1],
                    AHF_DENOMINATOR_COEF_50HZ[1],
                    AHF_OUTPUT_GAIN_COEF_50HZ[1],
                ),
            },
            NotchFrequency::Freq60Hz => match sample_freq {
                SampleFrequency::Freq500Hz => (
                    AHF_NUMERATOR_COEF_60HZ[0],
                    AHF_DENOMINATOR_COEF_60HZ[0],
                    AHF_OUTPUT_GAIN_COEF_60HZ[0],
                ),
                SampleFrequency::Freq1000Hz => (
                    AHF_NUMERATOR_COEF_60HZ[1],
                    AHF_DENOMINATOR_COEF_60HZ[1],
                    AHF_OUTPUT_GAIN_COEF_60HZ[1],
                ),
            },
        };
        Filter4th {
            states,
            num,
            den,
            gain,
        }
    }
    pub fn update(&mut self, input: f32) -> f32 {
        let stage_in = self.num[0] * input + self.states[0];
        self.states[0] = (self.num[1] * input + self.states[1]) - self.den[1] * stage_in;
        self.states[1] = self.num[2] * input - self.den[2] * stage_in;
        let stage_out = self.num[3] * stage_in + self.states[2];
        self.states[2] = (self.num[4] * stage_in + self.states[3]) - self.den[4] * stage_out;
        self.states[3] = self.num[5] * stage_in - self.den[5] * stage_out;
        self.gain * stage_out
    }
}

pub struct EMGFilters {
    sample_freq: SampleFrequency,
    notch_freq: NotchFrequency,
    bypass_enabled: bool,
    notch_filter_enabled: bool,
    lowpass_filter_enabled: bool,
    highpass_filter_enabled: bool,

    lpf: Filter2nd,
    hpf: Filter2nd,
    ahf: Filter4th,
}

impl EMGFilters {
    pub fn new(
        sample_freq: SampleFrequency,
        notch_freq: NotchFrequency,
        enable_notch_filter: bool,
        enable_lowpass_filter: bool,
        enable_highpass_filter: bool,
    ) -> Self {
        let bypass_enabled = !(((sample_freq == SampleFrequency::Freq500Hz)
            || (sample_freq == SampleFrequency::Freq1000Hz))
            && ((notch_freq == NotchFrequency::Freq50Hz)
                || (notch_freq == NotchFrequency::Freq60Hz)));
        EMGFilters {
            sample_freq,
            notch_freq,
            bypass_enabled,
            notch_filter_enabled: enable_notch_filter,
            lowpass_filter_enabled: enable_lowpass_filter,
            highpass_filter_enabled: enable_highpass_filter,

            lpf: Filter2nd::new(FilterType::LowPass, sample_freq),
            hpf: Filter2nd::new(FilterType::HighPass, sample_freq),
            ahf: Filter4th::new(sample_freq, notch_freq),
        }
    }

    pub fn update(&mut self, input_value: i32) -> i32 {
        let mut output = input_value as f32;

        if self.bypass_enabled {
            return input_value;
        }

        // first notch filter
        if self.notch_filter_enabled {
            output = self.ahf.update(output);
        }

        // second low pass filter
        if self.lowpass_filter_enabled {
            output = self.lpf.update(output);
        }

        // third high pass filter
        if self.highpass_filter_enabled {
            output = self.hpf.update(output);
        }

        output as i32
    }
}
