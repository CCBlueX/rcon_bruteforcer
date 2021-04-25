# RCON Bruteforcer
Allows you to bruteforce minecraft rcon server as quick as possible.

## Reminder
This is nothing to be used in the real world and a reminder to disable RCON.\
RCON is a very vulnerable way of accessing your server.\
Please access your server over a Dashboard or SSH into your server.

## Getting Started
Download latest binary from [releases](https://github.com/CCBlueX/RconBruteforcer/releases) or from the lastest commit.\
Execute binary: `./rcon_bruteforcer [127.0.0.1:25575]`\
View help: `./rcon_bruteforcer --help`

## Libraries
* [Tokio](https://crates.io/crates/tokio) - The IO async library
* [Futures](https://crates.io/crates/futures) - The future stream library
* [Bruteforce](https://crates.io/crates/bruteforce) - The password generator
* [Console](https://crates.io/crates/console) - The ANSI console library
* [Rcon](https://crates.io/crates/rcon) - The rcon connection library

## Authors
* **kawaiinekololis** - [GitHub](https://github.com/kawaiinekololis) / [Twitter](https://twitter.com/kawaiinekololis)

See also the list of [contributors](https://github.com/CCBlueX/RconBruteforcer/contributors) who participated in this project.