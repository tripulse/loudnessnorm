use structopt::StructOpt;
use hound::{WavReader, WavWriter};
mod dsp;

#[derive(StructOpt, Debug)]
#[structopt(version = "\0", about = "The loudness of the samples are normalized \
by calculating the RMS then the gain is changed to \
bring the average amplitude of the source signal to a target level.")]
struct RootOptions {
    #[structopt(about = "Input WAVE file to process", required = true)]
    input: String,

    #[structopt(help = "Output File to dump the data", required = true)]
    output: String,

    #[structopt(short, long, help = "Targetted average amplitude of signal", default_value = "1.0")]
    target_ampl: f32
}

fn main() {
    // Parameters to instruct the program's behaviour.
    // Basically, an interaction method for the CLI app.
    let root_params = RootOptions::from_args();

    // Input file to grab the data from.
    // The samples must be in the Float32 sample format.
    // Otherwise the program would panic.
    let mut wave_input = WavReader::open(root_params.input)
        .unwrap();

    // Output file to put the data into.
    let mut wave_output = WavWriter::create(
        root_params.output, wave_input.spec()
        ).unwrap();

    /* Apply the normalization filter by the DSP module. */
    let mut wave_samples = wave_input.samples::<f32>().map(Result::unwrap).collect();
    let signal_info = dsp::normalize_samples(&mut wave_samples, root_params.target_ampl).unwrap();

    /* Write each sample-data into the buffer which would be later
     * flushed to a file and that's the WAVE file of output
     */
    wave_samples.iter().for_each(|sample| wave_output
                                        .write_sample(*sample)
                                        .unwrap()
                        );

    /* some nitty-gritty details to the user */
    println!(
        "Gain applied:   {} dBFS\
        \nRMS of signal: {} dBFS",
        20.0 * signal_info.gain.log10(),
        20.0 * signal_info.rms.log10()
    );

    wave_output.finalize().unwrap();
}
