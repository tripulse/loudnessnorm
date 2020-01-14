use clap::{App, Arg, ArgMatches};

pub fn getargs<'a>() -> ArgMatches<'a> {
    App::new(crate_name!())
        .about(crate_description!())
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
