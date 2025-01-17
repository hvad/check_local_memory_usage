use clap::Parser;
use psutil::memory;
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    critical: u8,
    #[arg(short, long)]
    warning: u8,
}

fn main() {
    let args = Args::parse();
    let virtual_memory = memory::virtual_memory().unwrap();
    let memory_usage = virtual_memory.percent() as u8;

    match memory_usage {
        usage if usage > args.critical => {
            println!("CRITICAL - Memory usage {}%", usage);
            process::exit(2);
        }
        usage if usage > args.warning => {
            println!("WARNING - Memory usage {}%", usage);
            process::exit(1);
        }
        _ => {
            println!("OK - Memory usage {}%", memory_usage);
            process::exit(0);
        }
    }
}
