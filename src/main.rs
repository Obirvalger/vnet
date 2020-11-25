use std::io;

use clap::{App, AppSettings, Arg, Shell, SubCommand};

fn build_cli() -> clap::App<'static, 'static> {
    App::new("vnet")
        .version("0.1.0")
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::SubcommandRequired)
        .global_setting(AppSettings::InferSubcommands)
        .subcommand(
            SubCommand::with_name("completion")
                .about("Generate completion")
                .arg(Arg::with_name("SHELL").required(true).possible_values(
                    &["bash", "elvish", "fish", "powershell", "zsh"],
                )),
        )
        .subcommand(
            SubCommand::with_name("tap")
                .about("Set up tap devices")
                .setting(AppSettings::SubcommandRequired)
                .subcommand(
                    SubCommand::with_name("create")
                        .about("Create tap device")
                        .arg(Arg::with_name("NAME").required(true)),
                )
                .subcommand(
                    SubCommand::with_name("remove")
                        .about("Remove tap device")
                        .arg(Arg::with_name("NAME").required(true)),
                )
                .subcommand(
                    SubCommand::with_name("add-address")
                        .about("Add ip address to tap device")
                        .arg(Arg::with_name("NAME").required(true))
                        .arg(Arg::with_name("ADDRESS").required(true)),
                )
                .subcommand(
                    SubCommand::with_name("del-address")
                        .about("Delete ip address from tap device")
                        .arg(Arg::with_name("NAME").required(true))
                        .arg(Arg::with_name("ADDRESS").required(true)),
                ),
        )
        .long_about(
            "This is a helper to set up networks for virtual machines. To \
             work properly the program executable should be suid or has \
             CAP_NET_ADMIN capability.",
        )
}

fn process_completion(matches: &clap::ArgMatches<'static>) {
    fn print_completion(app: &mut App, shell: Shell) {
        app.gen_completions_to("vnet", shell, &mut io::stdout());
    }

    let mut app = build_cli();
    match matches.value_of("SHELL").unwrap() {
        "bash" => print_completion(&mut app, Shell::Bash),
        "elvish" => print_completion(&mut app, Shell::Elvish),
        "fish" => print_completion(&mut app, Shell::Fish),
        "powershell" => print_completion(&mut app, Shell::PowerShell),
        "zsh" => print_completion(&mut app, Shell::Zsh),
        _ => panic!("Unknown completion"),
    };
    std::process::exit(0);
}

fn process_tap(matches: &clap::ArgMatches<'static>) -> vnet::ExResult<()> {
    if let Some(matches) = matches.subcommand_matches("create") {
        let name = matches.value_of("NAME").unwrap();
        if let Some(new_name) = vnet::create_tap(name)? {
            println!("{}", new_name);
        }
    }
    if let Some(matches) = matches.subcommand_matches("remove") {
        let name = matches.value_of("NAME").unwrap();
        if let Some(new_name) = vnet::remove_tap(name)? {
            println!("{}", new_name);
        }
    }
    if let Some(matches) = matches.subcommand_matches("add-address") {
        let name = matches.value_of("NAME").unwrap();
        let address = matches.value_of("ADDRESS").unwrap();
        if let Some(new_address) = vnet::add_address_tap(name, address)? {
            println!("{}", new_address);
        }
    }
    if let Some(matches) = matches.subcommand_matches("del-address") {
        let name = matches.value_of("NAME").unwrap();
        let address = matches.value_of("ADDRESS").unwrap();
        if let Some(new_address) = vnet::del_address_tap(name, address)? {
            println!("{}", new_address);
        }
    }

    Ok(())
}

fn main() -> vnet::ExResult<()> {
    let matches = build_cli().get_matches();

    if let Some(matches) = matches.subcommand_matches("completion") {
        process_completion(matches);
    }

    vnet::set_ambient_cap()?;

    if let Some(matches) = matches.subcommand_matches("tap") {
        process_tap(matches)?;
    }

    Ok(())
}
