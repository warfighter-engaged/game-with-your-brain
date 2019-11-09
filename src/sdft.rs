use num::Complex;

type NumType = f64;

pub const DFT_LENGTH: usize = 512;
pub struct SlidingDFT {
    /// Are the frequency domain values valid? (i.e. have at least DFT_LENGTH data
    /// points been seen?)
    data_valid: bool,
    /// Time domain samples are stored in this circular buffer.
    x: [NumType; DFT_LENGTH],
    /// Index of the next item in the buffer to be used. Equivalently, the number
    /// of samples that have been seen so far modulo DFT_LENGTH.
    x_index: usize,
    /// Twiddle factors for the update algorithm.
    twiddle: [Complex<NumType>; DFT_LENGTH],
    /// Frequency domain values (unwindowed!)
    s: [Complex<NumType>; DFT_LENGTH],

    /// Frequency domain values (windowed)
    pub dft: [Complex<NumType>; DFT_LENGTH],
    /// A damping factor introduced into the recursive DFT algorithm to guarantee
    /// stability.
    pub damping_factor: NumType,
}

impl SlidingDFT {
    pub fn new() -> Self {
        let j = Complex::<NumType>::new(0f64, 1f64);
        let n: NumType = DFT_LENGTH as f64;

        let mut twiddle = [Complex::<NumType>::new(0f64, 0f64); DFT_LENGTH];
        let s = [Complex::<NumType>::new(0f64, 0f64); DFT_LENGTH];
        let x = [0f64; DFT_LENGTH];

        for k in 0..DFT_LENGTH {
            let factor = std::f64::consts::PI * 2f64 * (k as f64) / n;
            twiddle[k] = (j * factor).exp();
        }

        Self {
            data_valid: false,
            x,
            x_index: 0,
            twiddle,
            s,
            dft: [Complex::<NumType>::new(0f64, 0f64); DFT_LENGTH],
            damping_factor: float_extras::f64::nextafter(1f64, 0f64),
        }
    }

    /// Determine whether the output data is valid
    pub fn is_data_valid(&self) -> bool {
        self.data_valid
    }

    /// Update the calculation with a new sample.
	/// Returns true if the data is valid (because enough samples have been
	/// presented), or false if the data is invalid.
    pub fn update(&mut self, new_x: NumType) -> bool {
        let old_x = self.x[self.x_index];
        self.x[self.x_index] = new_x;

        // Update the DFT
        let r = self.damping_factor;
        let r_to_n = r.powf(DFT_LENGTH as f64);
        for k in 0..DFT_LENGTH {
            self.s[k] = self.twiddle[k] * (r * self.s[k] - r_to_n * old_x + new_x);
        }

        // Apply the Hanning window
        self.dft[0] = 0.5 * self.s[0] - 0.25*(self.s[DFT_LENGTH - 1] + self.s[1]);
        for k in 1..(DFT_LENGTH - 1) {
            self.dft[k] = 0.5*self.s[k] - 0.25*(self.s[k - 1] + self.s[k + 1]);
        }
        self.dft[DFT_LENGTH - 1] = 0.5 * self.s[DFT_LENGTH - 1] - 0.25 * (self.s[DFT_LENGTH - 2] + self.s[0]);

        // Increment the counter
        self.x_index += 1;
        if self.x_index >= DFT_LENGTH {
            self.data_valid = true;
            self.x_index = 0;
        }

        // Done.
        self.data_valid
    }

    pub fn get_value(&self) -> Option<Complex<NumType>> {
        if self.is_data_valid() {
            Some(self.dft[0])
        } else {
            None
        }
    }
}

