/// Information about the normalization process.
/// Contains, computed values while the process.
/// Use case is to, reduce the computation overhead.
pub struct NormalizationInfo {
    /// RMS (Root Mean Square) of all the samples in buffer.
    pub rms: f32,
    /// Gain applied to each sample which is calculated from the RMS.
    pub gain: f32
}

/// RMS based normalization function. This function calculates
/// the RMS and how much gain need to reach the targetted
/// average amplitude level.
///
/// Formula to calculate RMS is:
/// rms = sqrt(1/n * (x1**2 + x2**2 + ... + xn***2))
/// https://en.wikipedia.org/wiki/Root_mean_square
pub fn normalize_samples
(samples: &mut Vec<f32>, target_gain: f32) -> Result<NormalizationInfo, ()> {
    let signal_rms = f32::sqrt(samples.iter()
        .fold(0.0,|_s, s| _s + s.powf(2.0))
        / samples.len() as f32
    );
    let target_gain = target_gain / signal_rms;

    *samples = samples.iter().map(|sample| sample * target_gain).collect();
    Ok(NormalizationInfo { rms: signal_rms, gain: target_gain })
}