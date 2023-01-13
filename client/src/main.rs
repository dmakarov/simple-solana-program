use clap::{crate_description, crate_name, crate_version, Arg};
use client;

fn main() {
    let version = format!("{}", crate_version!());
    let args = std::env::args().collect::<Vec<_>>();
    let matches = clap::Command::new(crate_name!())
        .about(crate_description!())
        .version(version.as_str())
        .arg(
            Arg::new("config")
                .long("config")
                .short('C')
                .takes_value(true)
                .value_name("CONFIG")
                .help("Config filepath"),
        )
        .arg(
            Arg::new("keypair")
                .long("keypair")
                .short('k')
                .takes_value(true)
                .value_name("KEYPAIR")
                .help("Filepath or URL to a keypair"),
        )
        .arg(
            Arg::new("url")
                .long("url")
                .short('u')
                .takes_value(true)
                .value_name("URL_OR_MONIKER")
                .help("URL for Solana's JSON RPC or moniker (or their first letter): [mainnet-beta, testnet, devnet, localhost]"),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .takes_value(false)
                .help("Use verbose output"),
        )
        .get_matches_from(args);
    let config: Option<String> = matches.value_of_t("config").ok();
    let keypair: String = matches.value_of_t_or_exit("keypair");
    let url: Option<String> = matches.value_of_t("url").ok();
    let connection = client::client::establish_connection(&url, &config).unwrap();
    println!(
        "Connected to remote solana node running version ({}).",
        connection.get_version().unwrap()
    );
    let balance_requirement = client::client::get_balance_requirement(&connection).unwrap();
    println!(
        "({}) lamports are required for this transaction.",
        balance_requirement
    );
    let player = client::utils::get_player(&config).unwrap();
    let player_balance = client::client::get_player_balance(&player, &connection).unwrap();
    println!("({}) lamports are owned by player.", player_balance);
    if player_balance < balance_requirement {
        let request = balance_requirement - player_balance;
        println!(
            "Player does not own sufficent lamports. Airdropping ({}) lamports.",
            request
        );
        client::client::request_airdrop(&player, &connection, request).unwrap();
    }
    let program = client::client::get_program(&keypair, &connection).unwrap();
    client::client::create_greeting_account(&player, &program, &connection).unwrap();
    client::client::say_hello(&player, &program, &connection).unwrap();
    println!(
        "({}) greetings have been sent.",
        client::client::count_greetings(&player, &program, &connection).unwrap()
    )
}
