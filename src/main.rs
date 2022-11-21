use clap::Parser;
use psutil::memory;
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    critical: u8,
    #[arg(short, long)]
    warning: u8,
}

fn main() {
    let args = Args::parse();

    let virtual_memory = memory::virtual_memory().unwrap();

    if virtual_memory.percent() as u8 > args.critical {
        println!(
            "CRITICAL - Memory usage {}%", virtual_memory.percent() as u8
        );
        process::exit(2);
    } else if virtual_memory.percent() as u8 > args.warning {
        println!(
            "WARNING - Memory usage {}%", virtual_memory.percent() as u8
        );
        process::exit(1);
    } else {
        println!("OK - Memory usage {}%", virtual_memory.percent() as u8);
        process::exit(0);
    }

}
