pub mod DSPInfo {
    /* Contains precomputed values while applying the
       audio filter to the input signal. */
    pub struct NormalizationInfo {
        /* RMS (Root Mean Square) of all the samples in buffer. */
        pub rms: f32,
        
        /* Gain applied to each sample which is calculated from the RMS. */
        pub gain: f32
    }

    impl NormalizationInfo {
        /* Display the information on the Stdout. */
        pub fn display_info(self) {
            println!(
                "Gain (applied on input): {} dBFS\
                \nRMS (of the input):     {} dBFS",
                self.gain,
                self.rms
            );
        }
    }
}
/**
  * RMS based normalization function. This function calculates
  * the RMS and how much gain need to reach the targetted
  * average amplitude level.
  *
  * Formula to calculate RMS is:
  * rms = sqrt(1/n * (x1**2 + x2**2 + ... + xn***2))
  * https://en.wikipedia.org/wiki/Root_mean_square
  */
pub fn normalize_samples
(samples: &mut Vec<f32>, target_gain: f32) -> DSPInfo::NormalizationInfo {
    let signal_rms = f32::sqrt(samples.iter()
        .fold(0.0,|_s, s| _s + s.powf(2.0))
        / samples.len() as f32
    );

    /* Multiplier of samples to bring the average amplitude of
       them to a target gain. */
    let target_gain = target_gain / signal_rms;

    /* Replace the samples with the samples 
       normalized. */
    *samples = samples
                    .iter()
                    .map(|sample| sample * target_gain)
                    .collect();

    DSPInfo::NormalizationInfo { 
        rms: signal_rms,
        gain: target_gain
    }
}