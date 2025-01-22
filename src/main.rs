use clap::Parser;
use psutil::memory;
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'c', long, default_value = "75")]
    critical: u8,
    #[arg(short = 'w', long, default_value = "50")]
    warning: u8,
}

fn main() {
    let args = Args::parse();
    let memory_usage = memory::virtual_memory()
        .expect("Failed to retrieve memory information")
        .percent() as u8;

    let status = match memory_usage {
        usage if usage > args.critical => {
            println!("CRITICAL - Memory usage {}%", usage);
            2
        }
        usage if usage > args.warning => {
            println!("WARNING - Memory usage {}%", usage);
            1
        }
        _ => {
            println!("OK - Memory usage {}%", memory_usage);
            0
        }
    };

    process::exit(status);
}
