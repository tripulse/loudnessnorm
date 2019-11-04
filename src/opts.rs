use clap::{App, Arg, ArgMatches};

const _APP: &str = "loudnessnorm";
const _ABOUT: &str = "The loudness of the samples are normalized by calculating the \"RMS\" then the gain is changed to
bring the average amplitude of the source signal to a target level (by default `0 dBFS').";


/// Defines some arguments to control the bheaviour
/// or required the program to function.
/// 
/// NOTE: I found this method the most cleanest
///       and advanced one. So I'm using the method.
pub fn getargs<'a>() -> ArgMatches<'a> {
    App::new(_APP)
        .about(_ABOUT)
        .arg(
            Arg::with_name("INPUT")
            .help("Input file whose samples would be processed")
            .index(1)
            .required(true)
        )
        .arg(
            Arg::with_name("OUTPUT")
            .help("Output file where all the processes samples would be dumped")
            .index(2)
            .required(true)
        )
        .arg(
            Arg::with_name("target-amplitude")
            .long("target-ampl")
            .help("Target ampltiude to reach (linear power)")
            .takes_value(true)
        )
        .get_matches()
}
