/*
   Following are needed to set up the CLI arg parsing
   1. Command: The main CLI builder
   2. Arg: Individual argument builder
   3. ArgMatches: The parsed result we will extract from
*/

use clap::{Arg, ArgMatches, Command};

#[derive(Debug)]
pub struct ScanArguments {
    pub target_ip: String,
    pub ports: Option<String>,
    pub threads: usize,
}

impl ScanArguments {
    // Parses the CLI arguments
    pub fn arg_parse() -> Self {
        let matches: ArgMatches = Command::new("rustmap")
            .version(env!("CARGO_PKG_VERSION"))
            .author("Seb")
            .about("A minimalist port scanner")
            .arg(
                Arg::new("target")
                    .help("Target IP or hostname (e.g., 192.168.1.10 or example.com)")
                    .required(true)
                    .value_name("TARGET")
                    .index(1),
            )
            .arg(
                Arg::new("ports")
                .help("Enter Port (e.g., 80 or 1-1000. Leave it for default scan of first 10000 ports")
                .short('p')
                .long("ports")
                .value_name("PORTS")
                .required(false)
            )
            .arg(
                Arg::new("threads")
                .help("Thread Count")
                .short('t')
                .long("thread")
                .value_name("THREAD")
                .required(false)
                .default_value("100")
            ).get_matches();

        let target: String = matches
            .get_one::<String>("target")
            .expect("target is required")
            .to_string();

        let ports_value = matches.get_one::<String>("ports").map(|s| s.to_string());

        let threads = matches
            .get_one::<String>("threads")
            .expect("Dafault value missing")
            .parse::<usize>()
            .expect("Thread count must be a valid number");

        Self {
            target_ip: target,
            ports: ports_value,
            threads: threads,
        }
    }
}
