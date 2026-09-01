use template_test::TestConfig;
use clap::Arg;
use tracing::{info, warn};

fn make_args() -> Result<(TestConfig, String), String> {
    let matches = clap::Command::new("CCP Duo Test")
        .version("0.3.0")
        .author("Mengistie Hailemariam <mengistie_hailemariam@brown.edu>")
        .about("Test for having two different congestion controllers for CCP")
        .arg(Arg::new("ipc")
             .long("ipc")
             .help("Sets the type of ipc to use: (netlink|unix)")
             .default_value("unix")
             .value_parser(clap::builder::PossibleValuesParser::new(["netlink", "unix", "tcp"])))
        .get_matches();

    Ok((
        TestConfig {
            our_packets: 0,
            other_packets:0,
        },
        matches.get_one::<String>("ipc").unwrap().clone(),
    ))
}

fn main() {
    tracing_subscriber::fmt::init();
    let (cfg, ipc) = make_args()
        .map_err(|e| warn!(err = ?e, "bad argument"))
        .unwrap();
    portus::start!(ipc.as_str(), cfg).unwrap()
}
