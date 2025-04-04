use clap::Parser;
use psutil::memory;
use std::process;

#[derive(Parser, Debug)]
#[command(author = "David Hannequin", version = "1.0", about = "Check percent usage memory.")]
struct Args {
    #[arg(short = 'c', long, default_value = "75")]
    critical: u8,
    #[arg(short = 'w', long, default_value = "50")]
    warning: u8,
}

fn main() {
    let args = Args::parse();

    match memory::virtual_memory() {
        Ok(mem) => {
            let usage = mem.percent() as u8;

            let (status, message) = match usage {
                _ if usage > args.critical => (2, format!("CRITICAL - Memory usage {}%", usage)),
                _ if usage > args.warning => (1, format!("WARNING - Memory usage {}%", usage)),
                _ => (0, format!("OK - Memory usage {}%", usage)),
            };

            println!("{}", message);
            process::exit(status);
        }
        Err(e) => {
            eprintln!("ERROR - Failed to retrieve memory information: {}", e);
            process::exit(3); // Unknown error code 
        }
    }
}
