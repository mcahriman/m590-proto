extern crate serial_core as serial;
extern crate getopts;

use std::io;
use std::time::Duration;
use std::env;
use std::str;

use std::io::prelude::*;
use serial::prelude::*;

use getopts::Options;

const E_OK: i32 = 0;
const E_ERR: i32 = 1;
const DEFAULT_BAUD: u32 = 115200;
const DEFAULT_PORT: &str = "/dev/ttyUSB0";

struct AppConfig {
    port: String,
    baud_rate: u32,
}

fn main() {
    let config = get_configuration();
    println!("using port {}, baud {}", config.port, config.baud_rate);
}

fn get_configuration() -> AppConfig {
    let args: Vec<String> = env::args().collect();
    
    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optflag("h", "help", "Print this help menu");
    opts.optopt("b", "baud", "Baud rate", "115200");
    opts.optopt("p", "port", "port", "/dev/ttyUSB0");

    let matches = match opts.parse(&args) {
        Ok(result) => result,
        Err(f) => { panic!(f.to_string())},
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        std::process::exit(E_OK);
    };

    let port:String = match matches.opt_str("p") {
        None => String::from(DEFAULT_PORT),
        Some(x) => x
    };

    let baud:u32 = match matches.opt_str("b") {
        None => DEFAULT_BAUD,
        Some(x) => match x.parse() {
            Err(_) => {
                println!("Error parsing baud option");
                DEFAULT_BAUD
            },
            Ok(x) => x,
        },
    };
    
    AppConfig {
        port: port,
        baud_rate: baud,
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}
