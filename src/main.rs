use structopt::StructOpt;
use std::fs::File;
use std::io::{stdin,Read};
use hound::{WavReader, WavWriter};
mod dsp;

#[derive(StructOpt, Debug)]
#[structopt(version = "\0", about = "The loudness of the samples are normalized \
by calculating the RMS then the gain is changed to \
bring the average amplitude of the source signal to a target level.")]
struct RootOptions {
    #[structopt(help = "Input WAVE file to process (- as STDIN)", required = true)]
    input: String,

    #[structopt(help = "Output File to dump the data", required = true)]
    output: String,

    #[structopt(short, long, help = "Targetted average amplitude of signal", default_value = "1.0")]
    target_ampl: f32
}

fn main() {
    let _root_params = RootOptions::from_args();

    /**
     * Read the data from the STDIN or a *actual file*
     * from the filesystem.
     */
    let mut wave_input = WavReader::new(
        match _root_params.output.as_ref() {
            "-" => Box::new(stdin()) as Box<dyn Read>,
            _ => Box::new(File::create(&_root_params.output).unwrap()) as Box<dyn Read>
        }
    ).unwrap();

    /**
     * The sample writer, encodes the data into
     * the bistream and inherits the sample specifications
     * as the source file.
     */
    let mut wave_output = WavWriter::create(
        _root_params.output, wave_input.spec()
    ).unwrap();

    /**
     * This is where the real magic happens.
     * The signal sent to process via the DSP filter and later collected
     * as 32-bit floating point samples which is written to a WAVE file.
     */
    let mut wave_samples = wave_input.samples::<f32>().map(Result::unwrap).collect();
    let signal_info = dsp::normalize_samples(&mut wave_samples, _root_params.target_ampl)
                           .unwrap();

    /* Write each sample-data into the buffer which would be later
     * flushed to a file and that's the WAVE file of output
     */
    wave_samples.iter().for_each(|sample| wave_output
                                        .write_sample(*sample)
                                        .unwrap()
                        );

    // Verbose information, that a user might be eager
    // know about the source signal.
    println!(
        "Gain applied:   {} dBFS\
        \nRMS of signal: {} dBFS",
        20.0 * signal_info.gain.log10(),
        20.0 * signal_info.rms.log10()
    );

    wave_output.finalize().unwrap();
}