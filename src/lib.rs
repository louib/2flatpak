use std::collections::HashMap;

mod manifests;
mod execution_context;
mod utils;

use std::fs;
use std::path;

pub fn run(command_name: &str, args: HashMap<String, String>) -> i32 {
    eprintln!("running command {}.", command_name);

    if command_name == "convert" {
        if ! args.contains_key("input_file") {
            eprintln!("an input file is required when converting!");
            // TODO handle reading from stdin.
            return 1;
        }

        let input_file_path = args.get("input_file").unwrap();

        let fs_read_result = fs::read_to_string(path::Path::new(input_file_path));
        if fs_read_result.is_err() {
            eprintln!("could not read file {}.", input_file_path);
            return 1;
        }

        let mut ctx = crate::execution_context::ExecutionContext::default();
        ctx.content = fs_read_result.unwrap();

        if args.contains_key("input_format") {
            let source_type = args.get("input_format").unwrap();
            if ! crate::manifests::has_type(source_type.to_string()) {
                eprintln!("{} is an invalid manifest type.", source_type);
                return 1;
            }
            ctx.source_type = source_type.to_string();
        }

        if args.contains_key("output_format") {
            let destination_type = args.get("output_format").unwrap();
            if ! crate::manifests::has_type(destination_type.to_string()) {
                eprintln!("{} is an invalid manifest type.", destination_type);
                return 1;
            }
            ctx.destination_type = destination_type.to_string();
        }

        let mut exit_code: i32 = manifests::get_type(&ctx);
        if exit_code != 0 {
            return exit_code;
        }

        exit_code = manifests::parse(&ctx);
        if exit_code != 0 {
            return exit_code;
        }

        exit_code = manifests::dump(&ctx);
        if exit_code != 0 {
            return exit_code;
        }

        return 0;
    }

    if command_name == "ls" {
        return 0;
    }

    eprintln!("Finishing...");
    return 0;
}
