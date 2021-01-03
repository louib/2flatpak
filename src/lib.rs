//! # Panbuild
//!
//! `panbuild` is the universal builder.
use std::collections::HashMap;

mod execution_context;
mod manifests;
mod projects;
mod utils;

pub use manifests::manifest::AbstractManifest;
pub use manifests::manifest::AbstractModule;

use std::env;
use std::fs;
use std::path;

const DEFAULT_CACHE_DIR: &str = ".panbuild/";
const DEFAULT_GIT_CACHE_DIR: &str = ".git/";
const DEFAULT_FLATPAK_BUILDER_CACHE_DIR: &str = ".flatpak-builder/";
const DEFAULT_FLATPAK_BUILD_CACHE_DIR: &str = ".build/";
const DEFAULT_PACKAGE_LIST_SEP: &str = ",";

struct PanbuilbArguments {
    // TODO use enum for command name?
    command_name: String,
    arguments: Vec<String>,
    // TODO use enums for those?
    input_format: String,
    output_format: String,
}

pub fn run(command_name: &str, args: HashMap<String, String>) -> i32 {
    // FIXME put to debug once there is proper logging in place
    // eprintln!("running command {}.", command_name);
    let mut ctx = crate::execution_context::ExecutionContext::default();

    let mut config = match crate::execution_context::read_or_init_config() {
        Ok(c) => c,
        Err(e) => panic!("Could not load or init config: {}", e),
    };

    if command_name == "lint" {
        let input_file_path = args.get("input_file").expect("an input file is required!");

        let abstract_manifest = match crate::manifests::manifest::AbstractManifest::load_from_file(input_file_path.to_string()) {
            Some(m) => m,
            None => return 1,
        };

        ctx.manifest = abstract_manifest;
        eprintln!("Parsed abstract manifest finished. Resulting manifest is {:#?}", &ctx.manifest);

        let manifest_dump = match ctx.manifest.dump() {
            Ok(d) => d,
            Err(e) => return 1,
        };

        match fs::write(path::Path::new(input_file_path), manifest_dump) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("could not write file {}.", input_file_path);
                return 1;
            },
        };

        eprintln!("Dumped the manifest!");
        return 0;
    }

    if command_name == "get-package-list" {
        let input_file_path = args.get("input_file").expect("an input file is required!");

        ctx.content = match fs::read_to_string(path::Path::new(input_file_path)) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("could not read file {}.", input_file_path);
                return 1;
            }
        };

        let input_format = args.get("input_format").unwrap();
        if !input_format.is_empty() {
            if !crate::manifests::has_type(input_format.to_string()) {
                eprintln!("{} is an invalid manifest type.", input_format);
                return 1;
            }
            ctx.source_type = input_format.to_string();
        } else {
            let mut exit_code: i32 = manifests::detect_type(&mut ctx);
            if exit_code != 0 {
                eprintln!("Could not detect manifest type of {}.", ctx.source_filename);
                return exit_code;
            }
        }

        let mut exit_code: i32 = manifests::parse(&mut ctx);
        if exit_code != 0 {
            eprintln!("Error while parsing");
            return exit_code;
        }

        eprintln!("Parsing finished. Resulting manifest is {:#?}", &ctx.manifest);

        let mut separator = DEFAULT_PACKAGE_LIST_SEP;
        if args.contains_key("separator") {
            separator = args.get("separator").unwrap();
        }

        exit_code = manifests::get_modules(&mut ctx);
        if exit_code != 0 {
            eprintln!("Error while getting modules");
            return exit_code;
        }

        let mut output: String = String::from("");
        for module in &ctx.manifest.depends_on {
            if !output.is_empty() {
                output.push_str(&separator)
            }
            output.push_str(&module.name);
        }
        println!("{}", output);
    }

    if command_name == "search" {
        let search_term = match args.get("search_term") {
            Some(search_term) => search_term,
            None => {
                eprintln!("A search term is required!");
                return 1;
            }
        };
        if search_term.len() < 3 {
            eprintln!("{} is too short for a search term!", search_term);
            return 1;
        }
        eprintln!("Search for {} in the projects database.", &search_term);

        let packages: Vec<crate::manifests::manifest::AbstractModule> = crate::projects::get_modules();
        eprintln!("Searching in {:#?} packages for installation candidates 🕰", packages.len());
        for package in &packages {
            if package.name.contains(search_term) {
                println!("found candidate artifact in {}.", package.name);
            }
        }
    }

    if command_name == "install" {
        let workspace_name = match &config.current_workspace {
            Some(w) => w,
            None => {
                eprintln!("Not currently in a workspace. Use `ls` to list the available workspaces and manifests.");
                return 1;
            }
        };

        if !config.workspaces.contains_key(workspace_name) {
            eprintln!("Workspace {} does not exist. Use `ls` to list the available workspaces and manifests.", workspace_name);
            return 1;
        }

        ctx.source_filename = config.workspaces.get(workspace_name).unwrap().to_string();
        println!("Using source_filename {}", &ctx.source_filename);

        ctx.content = match fs::read_to_string(path::Path::new(&ctx.source_filename)) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("could not read file {}.", &ctx.source_filename);
                return 1;
            }
        };

        let input_format = args.get("input_format").unwrap();
        if !input_format.is_empty() {
            if !crate::manifests::has_type(input_format.to_string()) {
                eprintln!("{} is an invalid manifest type.", input_format);
                return 1;
            }
            ctx.source_type = input_format.to_string();
        } else {
            let mut exit_code: i32 = manifests::detect_type(&mut ctx);
            if exit_code != 0 {
                eprintln!("Could not detect manifest type of {}.", ctx.source_filename);
                return exit_code;
            }
        }

        let mut exit_code: i32 = manifests::parse(&mut ctx);
        if exit_code != 0 {
            eprintln!("Error while parsing");
            return exit_code;
        }

        eprintln!("Parsing finished. Resulting manifest is {:#?}", &ctx.manifest);

        let package_name = match args.get("package_name") {
            Some(package_name) => package_name,
            None => {
                eprintln!("A package name to install is required!");
                return 1;
            }
        };
        if package_name.len() < 3 {
            eprintln!("{} is too short for a package name!", package_name);
            return 1;
        }
        eprintln!("Installing module {:#?}", &package_name);

        let packages: Vec<crate::manifests::manifest::AbstractModule> = crate::projects::get_modules();
        let mut installed_package: Option<&crate::manifests::manifest::AbstractModule> = None;
        eprintln!("Searching in {:#?} packages for installation candidates 🕰", packages.len());
        for package in &packages {
            if package.name.contains(package_name) {
                println!("found candidate artifact in {}.", package.name);
                let question = format!("Do you want to install {} ({})", package.name, package.url);
                if crate::utils::ask_yes_no_question(question) {
                    println!("installing {}.", package.name);
                    crate::manifests::add_module(&mut ctx, package);
                    installed_package = Some(package);
                    break;
                }
            }
        }

        let installed_package_name = match installed_package {
            Some(p) => p,
            None => {
                println!("Did not install any package.");
                return 1;
            }
        };
        let installed_package_name = &installed_package_name.name;
        println!("Installed package {}.", installed_package_name);

        exit_code = manifests::dump(&mut ctx);
        if exit_code != 0 {
            eprintln!("Error while dumping");
            return exit_code;
        }

        match fs::write(path::Path::new(&ctx.source_filename), ctx.content) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("could not write file {}.", &ctx.source_filename);
                return 1;
            }
        };
        return 0;
    }

    if command_name == "ls" {
        let git_cache_dir = path::Path::new(DEFAULT_GIT_CACHE_DIR);
        if !git_cache_dir.is_dir() {
            eprintln!("This does not seem like a git project (.git/ was not found).");
            return 1;
        }

        let mut found_manifest = false;
        let file_paths = match utils::get_all_paths(path::Path::new("./")) {
            Ok(paths) => paths,
            Err(message) => {
                eprintln!("Could not get the file paths :sad: {}", message);
                return 1;
            }
        };
        for path in file_paths.iter() {
            let file_path = path;
            let file_path_str = file_path.to_str().unwrap();
            if file_path.is_dir() {
                continue;
            }

            // TODO Test that if it starts with the cache directories listed above,
            // you skip the file.

            if crate::manifests::debian::file_path_matches(file_path_str) {
                found_manifest = true;
                println!("debian ({})", file_path_str);
            }
            if crate::manifests::snap::file_path_matches(file_path_str) {
                found_manifest = true;
                println!("snap ({})", file_path_str);
            }
            if crate::manifests::flatpak::file_path_matches(file_path_str) {
                found_manifest = true;
                println!("flatpak ({})", file_path_str);
            }
        }

        if !found_manifest {
            eprintln!("No available workspace found for the project. Try running `ls -p`.");
        } else {
            println!("Use `checkout` to select a workspace.");
        }
    }

    if command_name == "checkout" {
        let env_name = match args.get("env_name") {
            Some(n) => n,
            None => panic!("An env name is required to checkout."),
        };

        if let Some(current_workspace) = &config.current_workspace {
            if current_workspace == env_name {
                println!("Already in workspace {}.", env_name);
                return 0;
            }
        }

        if !config.workspaces.contains_key(env_name) {
            eprintln!("Workspace {} does not exist. Use `ls` to list the available workspaces and manifests.", env_name);
            return 1;
        }

        config.current_workspace = Some(env_name.to_string());
        match crate::execution_context::write_config(&config) {
            Ok(c) => c,
            Err(e) => panic!("Could not write config: {}", e),
        };
    }

    if command_name == "create" {
        let env_name = match args.get("env_name") {
            Some(n) => n,
            None => panic!("An env name is required to checkout."),
        };

        if let Some(current_workspace) = &config.current_workspace {
            if current_workspace == env_name {
                println!("Already in workspace {}.", env_name);
                return 0;
            }
        }

        if config.workspaces.contains_key(env_name) {
            eprintln!("Workspace {} already exists.", env_name);
            return 1;
        }

        let manifest_file_path = match args.get("manifest_file_path") {
            Some(p) => p,
            None => {
                eprintln!("a manifest file is required to create a new workspace!");
                // TODO handle reading from stdin.
                return 1;
            }
        };

        config.workspaces.insert(env_name.to_string(), manifest_file_path.to_string());
        config.current_workspace = Some(env_name.to_string());
        match crate::execution_context::write_config(&config) {
            Ok(c) => c,
            Err(e) => panic!("Could not write config: {}", e),
        };
        println!("🗃 Created workspace {} with manifest file {}.", env_name, manifest_file_path);
    }

    if command_name == "status" {
        let current_workspace = match config.current_workspace {
            Some(workspace) => workspace,
            None => "".to_string(),
        };

        if current_workspace.len() == 0 {
            println!("Not current in a workspace. Call `ls` to list the available workspaces and manifest files.");
            return 0;
        }

        if !config.workspaces.contains_key(&current_workspace) {
            panic!("Workspace {} not found in config!.", current_workspace);
            return 1;
        }

        let manifest_file_path = config.workspaces.get(&current_workspace).unwrap();
        println!("Current workspace is {} ({}).", current_workspace, manifest_file_path);
    }
    // FIXME put to debug once there is proper logging in place
    // eprintln!("Finishing...");
    return 0;
}
