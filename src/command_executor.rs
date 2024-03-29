use std::io::Write;
use std::io;
use colored::*;
use crate::entities::command::{Command, COMMANDS, display_help};
use crate::entities::Dotfile;
use crate::factories::dotfile_factory;

pub fn run(command: &str, file_name: Option<&str>) {
    let command_info = COMMANDS.iter().find(|c| c.name == command).unwrap();

    match command_info.command {
        Command::Apply => apply(file_name),
        Command::Sync => sync(file_name),
        Command::Clean => clean(file_name),
        Command::CleanSelf => clean_me(file_name),
        Command::Help => display_help(),
    }
}


pub fn apply(file_name: Option<&str>) {
    run_on_all_data(Dotfile::apply, "apply", file_name);
}

pub fn sync(file_name: Option<&str>) {
    run_on_all_data(Dotfile::sync, "sync", file_name);
}

pub fn clean(file_name: Option<&str>) {
    run_on_all_data(Dotfile::clean, "clean", file_name);
}

pub fn clean_me(file_name: Option<&str>) {
    run_on_all_data(Dotfile::clean_me, "clean_me", file_name);
}

fn run_on_all_data<F: Fn(&Dotfile)>(operation: F, operation_name: &str, file_name: Option<&str>) {
    println!("starting {}...", operation_name);

    let mut exec_all = false;

    let dotfiles_data = dotfile_factory::create_from_mappings();
    let dotfiles_data = match file_name {
        Some(grep) => {
            dotfiles_data.into_iter().filter(|dotfile| dotfile.path_behavior.from().contains(grep)).collect()
        },
        None => dotfiles_data,
    };

    for dotfile in &dotfiles_data {
        if exec_all {
            operation(dotfile);
            continue;
        }

        let mut command = String::new();
        loop {
            print!("> {} to {} (y, n, d, a, q, h): ", operation_name.bold(), dotfile.path_behavior.from().red().bold());

            io::stdout().flush().unwrap();
            command.clear();
            io::stdin().read_line(&mut command).unwrap();
            let current_command: String = command.trim().parse().expect("文字列を入力してください");

            match current_command.as_str() {
                "y" => operation(dotfile),
                "n" => {},
                "a" => {
                    exec_all = true;
                    operation(dotfile)
                },
                "q" => {
                    println!("quit");
                    std::process::exit(0)
                },
                "h" => {
                    print_help_message();
                    continue;
                },
                "d" => {
                    dotfile.diff();
                    continue;
                },
                _ => {
                    println!("{} is not found\n", current_command);
                    print_help_message();
                    continue;
                },
            }

            break
        }
    }
}

fn print_help_message() {
    println!("help:");
    println!("    y: execute this file");
    println!("    n: skip this file");
    println!("    a: execute all files");
    println!("    q: quit");
    println!("    h: help");
}
