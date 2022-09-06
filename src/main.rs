mod lib;
use crate::lib::service::commands::start;
use crate::lib::service::services_cli;
use crate::lib::service::system;
use clap::Parser;
use std::process::exit;

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    service: Service,
}

#[derive(clap:: Subcommand, Debug)]
enum Service {
    #[clap(subcommand)]
    Services(Action),
}

#[derive(clap:: Subcommand, Debug)]
enum Action {
    /// Start the service <formula> immediately and register it to launch at login (or boot)
    Start { formula: Option<String> },

    /// List information about all managed services for the current user (or root).
    List,

    /// Stop the service <formula> immediately and unregister it from launching at login (or
    /// boot)
    Stop { formula: Option<String> },
}

fn main() {
    if !system::check_if_launchctl_exists().exists() {
        panic!("This is not common to see but somehow `launchctl` is missing from the system")
    }

    let args = Args::parse();

    match args.service {
        Service::Services(Action::List) => println!("Volla show list"),
        Service::Services(Action::Start { ref formula }) => {
            if formula.is_none() {
                println!("Formula(e) missing, please provide a formula name");
                exit(1);
            }
            services_cli::service_load(formula.as_ref().unwrap().as_str());
        }
        _ => print!("Oops!"),
    }
}
