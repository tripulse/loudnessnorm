#[allow(dead_code, unused_doc_comments, unused_imports)]

mod opts;
use hound::{WavReader, WavWriter};
mod dsp;

fn main() {
    /* 
        Stores arguments parsed from the commandline.
        This functions in the similar as the GNU argparse does.

        NOTE: from similar I mean that clap is bit different
              than the traditional argparse.
    */
    let args = opts::getargs();

    /* Opens a file specified in arguments from the filesystem
       this handles the IO operation internally.

       NOTE: for strange reason this doesn't work with
             pipes like STDIN. For the sake of the code
             being functional the feature is supressed.
    */
    let mut wave_input = WavReader::open(
        args.value_of("INPUT").unwrap()
    ).unwrap();

    /* 
        Creates a file (overwrites if exists) in the filesystem
        this also handles the IO job itself.

        NOTE: unlike the reader this requires Seek trait
              to present with the Write trait. Like
              STDIN, STDOUT isn't capable of seeking.
    */
    let mut wave_output = WavWriter::create(
        args.value_of("OUTPUT").unwrap(), wave_input.spec()
    ).unwrap();

    /* 
        Collect all the samples from the WAVE file.
        The samples must be in the 32-bit floating point
        format otherwise it would fail to work.

        NOTE: this is reason of why we can't use pipes,
              we can't read all the samples from a single pipe
    */
    let mut wave_samples = wave_input.samples::<f32>()
                            .map(Result::unwrap).collect();

    /* 
        Samples' ownership is transferred to the filter
        and the actual processing is done. The target gain
        is by default (0 dBFS).
    */
    dsp::normalize_samples(&mut wave_samples, 
                            args.value_of("target-amplitude").unwrap()
                            .parse::<f32>().unwrap()
        ).display_info();

    
    /*
        Write the processed samples to the output file.
        The samples are stored in a buffer to be
        later flushed along with the bitstream.
    */
    wave_samples.iter().for_each(|sample| wave_output
                                        .write_sample(*sample)
                                        .unwrap());

    wave_output.finalize().unwrap();
}