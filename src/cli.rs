use clap::{self, Arg, App, SubCommand, ArgMatches};

pub fn parse_args() -> ArgMatches<'static> {
    let matches = App::new("ODIN - OpenWrt Device Information")
        .version("1.0")
        .author("Meenakshi Sundaram V <vms20591@riseup.net>")
        .about("CLI for OpenWrt's supported devices page")
        .subcommand(
            SubCommand::with_name("list")
                .about("Lists all available router brands")
                .arg(
                    Arg::with_name("models")
                        .short("m")
                        .long("models")
                        .help("Lists all available model details for a router brand.\nEffective only with -b/--brand or -a/-all option.")
                )
                .arg(
                    Arg::with_name("brand")
                        .short("b")
                        .long("brand")
                        .help("Brand to list all model details for.\nEffective only with -m/--models option.")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("all")
                        .short("a")
                        .long("all")
                        .help("Lists all available model detals for all router brands.\nOverrides -n/--name option.\nEffective only with -m/--models option.")
                )
        )
        .get_matches();

        matches
}