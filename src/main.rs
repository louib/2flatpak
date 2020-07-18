use std::collections::HashMap;
// TODO tune built-in attributes
// From https://doc.rust-lang.org/reference/items/modules.html#attributes-on-modules
// The built-in attributes that have meaning on a module are cfg, deprecated, doc,
// the lint check attributes, path, and no_implicit_prelude.
// Modules also accept macro attributes.
extern crate clap;

use clap::{Arg, App, ArgMatches, SubCommand};
use std::fs;
use std::path;
use std::process::{exit};

mod manifest;
mod snap;
mod npm;
mod flatpak;
mod debian;
mod pyproject;
mod utils;

fn main() {
    let pandoc_app: App = App::new("panbuild")
                          .version("0.0.1")
                          .author("louib <code@louib.net>")
                          .about("The universal build manifest converter.")
                          .arg(Arg::with_name("version")
                               .short("V")
                               .long("version")
                               .required(false)
                               .help("Show the version and exit."))
                          .subcommand(SubCommand::with_name("convert")
                               .about("convert a manifest file.")
                               .arg(Arg::with_name("input_file")
                                    .short("i")
                                    .long("input-file")
                                    .takes_value(true)
                                    .value_name("MANIFEST")
                                    .required(false)
                                    .help("Path of the input build manifest.")))
                          .subcommand(SubCommand::with_name("spec")
                               .about("Show the spec for a manifest type."));
    let matches: ArgMatches = pandoc_app.get_matches();

    if matches.is_present("version") {
        println!("0.0.1");
        exit(0);
    }

    let command_name = matches.subcommand_name().unwrap();

    //if ! command_name {
    //    println!("please specify a command.");
    //    // TODO show help.
    //    exit(1)
    //}
    match matches.subcommand_name() {
        Some(command_name)   => {
            if let Some(subcommand_matches) = matches.subcommand_matches(command_name) {
                let exit_code = run(command_name, subcommand_matches);
                exit(exit_code);
            }
        },
        None => {
            // TODO this should print to stderr.
            println!("Please provide a command to execute.");
            exit(1);
        },
        _ => {
            // TODO this should print to stderr.
            println!("Unknown command {0}.", command_name);
            exit(1);
        },
    }


}

fn run(command_name: &str, args: &ArgMatches) -> i32 {
    println!("running command {}.", command_name);

    if command_name == "convert" {
        if ! args.is_present("input_file") {
            // TODO handle reading from stdin.
            return 0;
        }

        let input_file = args.value_of("input_file").unwrap();
        let input_file_path = path::Path::new(input_file);

        let manifest_content = fs::read_to_string(input_file_path).unwrap();

        let manifest_type: &str = "snap";
        manifest::get_type(input_file.to_string(), manifest_type);

        let manifest = manifest::get_manifest(manifest_content, manifest_type.to_string());
        return 0;
    }

    if command_name == "ls" {
        return 0;
    }

    return 0;
}
