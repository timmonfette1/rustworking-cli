/* rustworking-cli
 *
 * Command line tool to handle network tasks for
 * system administration. Has the ability to perform
 * various tasks in bulk
 *
 * Currently supports:
 *      PING an IP address (single or in bulk)
 *
 * Coming soon:
 *      Send a TCP packet to test a connection
 *      Send a UDP packet to test a connection
 *
 * Author: Tim Monfette
 * Version: 0.1.0
 */

extern crate argparse;
extern crate rustworking_core;

use argparse::{ArgumentParser, Store, StoreTrue, Print};
use rustworking_core::rustytools;

use std::env;
use std::process::exit;
use std::io::prelude::*;
use std::str::from_utf8;
use std::ascii::AsciiExt;

// Struct for the accepted options
struct Options {
    verbose: bool,
    tool: String,
    ip: String,
    port: String,
    subnet: String,
    filepath: String,
}

fn main() { 
    // Set default values
    let mut options = Options {
        verbose: false,
        tool: String::new(),
        ip: String::from("localhost"),
        port: String::from("80"),
        subnet: String::new(),
        filepath: String::new()
    }; 

    // For getting the usage message when building the argparser
    let help: String;
    let mut buf = Vec::<u8>::new();
    let args: Vec<String> = env::args().collect(); 

    {   // Open scope

        let mut ap = ArgumentParser::new();
        ap.set_description("Test connections to a server or set of servers.");

        ap.add_option(&["-V", "--version"],
                      Print(format!("Current version of rustworking: {}",
                                    env!("CARGO_PKG_VERSION").to_string())), "Show version");

        ap.refer(&mut options.verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
                        "Verbose execution");

        ap.refer(&mut options.tool)
            .add_option(&["-t", "--tool"], Store,
                        "Network tool to use [ping, http, tcp, udp]");

        ap.refer(&mut options.ip)
            .add_option(&["-i", "--ip"], Store,
                        "IP Address of server");

        ap.refer(&mut options.port)
            .add_option(&["-p", "--port"], Store,
                        "Port to test connection on");

        ap.refer(&mut options.subnet)
            .add_option(&["-s", "--subnet"], Store,
                        "Subnet of addresses to test on");

        ap.refer(&mut options.filepath)
            .add_option(&["-f", "--filepath"], Store,
                        "Path to file of IP addresses");

        // Get the usage message
        if !ap.print_help(&args[0], &mut buf).is_ok() {
            let mut stderr = std::io::stderr();
            writeln!(&mut stderr, "Could not  build help message.\n'argparser' not functioning correctly."
                    ).expect("Could not write to stderr");
            exit(1);
        }

        help = from_utf8(&buf[..]).unwrap().to_string();

        ap.parse_args_or_exit(); 

    } // end scope

    // Make sure at least 1 argument is passed
    if options.tool.is_empty() {
        let mut stderr = std::io::stderr();
        writeln!(&mut stderr, "rustworking: No tool specified.\n{}",
                 help).expect("Could not write to stderr");
        exit(1);
    }

    // Handle verbose execution
    if options.verbose {
        println!("Beginning execution...");
        println!("Tool: {}", options.tool);
        println!("IP Address: {}", options.ip);
        println!("Port: {}", options.port);
        println!("Subnet: {}", options.subnet);
        println!("Filepath: {}", options.filepath);
    }

    // Run the correct networking tool
    match Some(&*options.tool.to_string().to_ascii_lowercase()) {
        Some("ping")  => ping_helper(options.verbose, &options.ip,
                                    &options.subnet, &options.filepath),
        Some("http")  => http_helper(options.verbose, &options.ip,
                                     &options.subnet, &options.filepath),
        Some("tcp")   => println!("Tool: tcp"),
        Some("udp")   => println!("Tool: udp"),
        _             => 
        { let mut stderr = std::io::stderr();
            writeln!(&mut stderr, "rustworking: Unrecognized tool '{}'.\n{}",
                     options.tool, help).expect("Could not write to stderr");
            exit(1);
        },
    }
}

// Function to help run a PING
fn ping_helper(verbose: bool, ip: &str, subnet: &str, filepath: &str) {
    if !filepath.is_empty() {
        let results = rustytools::ping_file(verbose, filepath);
        for res in results {
            match res {
                Ok(r)  => println!("{}", r),
                Err(e) => println!("{}", e),
            }
        }
    } else if !subnet.is_empty() { 
        let results = rustytools::ping_subnet(verbose, subnet);
        for res in results {
            match res {
                Ok(r)  => println!("{}", r),
                Err(e) => println!("{}", e),
            }
        }
    } else {
        match rustytools::ping_ip(verbose, ip) {
            Ok(r)  => println!("{}", r),
            Err(e) => println!("{}", e),
        }
    }
}

// Function to help run an HTTP request
fn http_helper(verbose: bool, ip: &str, subnet: &str, filepath: &str) {
    if !filepath.is_empty() {
        let results = rustytools::http_file(verbose, filepath);
        for res in results {
            match res {
                Ok(r)  => println!("{}", r),
                Err(e) => println!("{}", e),
            }
        }
    } else if !subnet.is_empty() {
        let results = rustytools::http_subnet(verbose, subnet);
        for res in results {
            match res {
                Ok(r)  => println!("{}", r),
                Err(e) => println!("{}", e),
            }
        }
    } else {
        match rustytools::http_ip(verbose, ip) {
            Ok(r)  => println!("{}", r),
            Err(e) => println!("{}", e),
        }
    }
}
