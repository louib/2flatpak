use std::env;
use std::fs;
use std::path;

use uuid::Uuid;

pub const DEFAULT_DB_PATH: &str = ".panbuild-db";
pub const MODULES_DB_SUBDIR: &str = "/modules";
pub const PROJECTS_DB_SUBDIR: &str = "/projects";

pub struct Database {
    pub projects: Vec<crate::projects::project::Project>,
    pub modules: Vec<crate::modules::SoftwareModule>,
}
impl Database {
    pub fn get_database() -> Database {
        if let Err(e) = fs::create_dir_all(Database::get_modules_db_path()) {
            panic!("Could not initialize database directory: {}.", e);
        }
        if let Err(e) = fs::create_dir_all(Database::get_projects_db_path()) {
            panic!("Could not initialize database directory: {}.", e);
        }
        // FIXME error handle the init.
        Database {
            projects: Database::get_all_projects(),
            modules: Database::get_all_modules(),
        }
    }

    pub fn get_db_path() -> String {
        if let Ok(path) = env::var("PB_DB_PATH") {
            return path.to_string();
        }
        if let Ok(path) = env::var("HOME") {
            return path + "/" + &DEFAULT_DB_PATH.to_string();
        }
        return DEFAULT_DB_PATH.to_string();
    }

    pub fn get_modules_db_path() -> String {
        Database::get_db_path() + MODULES_DB_SUBDIR
    }

    pub fn get_projects_db_path() -> String {
        Database::get_db_path() + PROJECTS_DB_SUBDIR
    }

    pub fn get_all_projects() -> Vec<crate::projects::project::Project> {
        let json_projects_db_path = env::var("PB_PROJECTS_DB_PATH").unwrap_or(String::from("")).to_string();
        if json_projects_db_path.is_empty() {
            return vec![];
        }

        let json_projects = match fs::read_to_string(&json_projects_db_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Could not read file {}.", json_projects_db_path);
                return vec![];
            }
        };
        let mut db_projects: Vec<crate::projects::project::Project> = serde_json::from_str(&json_projects).unwrap();

        db_projects
    }

    pub fn get_all_modules() -> Vec<crate::modules::SoftwareModule> {
        let modules_path = Database::get_modules_db_path();
        let modules_path = path::Path::new(&modules_path);
        let all_modules_paths = match crate::utils::get_all_paths(modules_path) {
            Ok(paths) => paths,
            Err(e) => {
                return vec![];
            }
        };
        let mut modules: Vec<crate::modules::SoftwareModule> = vec![];
        for module_path in all_modules_paths.iter() {
            let module_path_str = module_path.to_str().unwrap();
            if !module_path.is_file() {
                log::debug!("{} is not a file.", &module_path_str);
                continue;
            }
            // Don't even try to open it if it's not a yaml file.
            if !module_path_str.ends_with("yml") && !module_path_str.ends_with("yaml") {
                continue;
            }
            let module_content = match fs::read_to_string(module_path) {
                Ok(content) => content,
                Err(e) => {
                    log::debug!("Could not read module file {}: {}.", &module_path_str, e);
                    continue;
                }
            };
            let module = match serde_yaml::from_str(&module_content) {
                Ok(m) => m,
                Err(e) => {
                    log::debug!("Could not parse module file at {}: {}.", &module_path_str, e);
                    continue;
                }
            };
            modules.push(module);
        }
        modules
    }

    pub fn search_modules(&self, search_term: &str) -> Vec<&crate::modules::SoftwareModule> {
        let mut modules: Vec<&crate::modules::SoftwareModule> = vec![];
        for module in &self.modules {
            if module.name.contains(&search_term) {
                modules.push(&module);
            }
        }
        modules
    }

    pub fn remove_module() {}

    pub fn add_module(&mut self, mut new_module: crate::modules::SoftwareModule) {
        let new_uuid = Uuid::new_v4();
        new_module.id = Some(new_uuid.to_string());
        let modules_path = Database::get_modules_db_path();
        let mut new_module_path = format!(
            "{}/{}-{}.yaml",
            modules_path,
            crate::utils::normalize_name(&new_module.name),
            new_module.id.as_ref().unwrap()
        );
        log::info!("Adding module at {}", new_module_path);
        let mut new_module_fs_path = path::Path::new(&new_module_path);
        if new_module_fs_path.exists() {
            panic!("Path {} already exists. This should not happen!!", new_module_path);
        }
        match fs::write(new_module_fs_path, serde_yaml::to_string(&new_module).unwrap()) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Could not write new module at {}: {}", new_module_path.to_string(), e);
            }
        };
        self.modules.push(new_module);
    }

    pub fn add_project(&mut self, mut project: crate::projects::project::Project) {
        let projects_path = Database::get_projects_db_path();
        if project.id.len() == 0 {
            panic!("Trying to add a project to the db without an id!");
        }
        let mut new_project_path = format!(
            "{}/{}.yaml",
            projects_path,
            &project.id,
        );
        log::info!("Adding project at {}", new_project_path);
        let mut new_project_fs_path = path::Path::new(&new_project_path);
        if new_project_fs_path.exists() {
            panic!("Path {} already exists. This should not happen!!", new_project_path);
        }
        match fs::write(new_project_fs_path, serde_yaml::to_string(&project).unwrap()) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Could not write new project at {}: {}", new_project_path.to_string(), e);
            }
        };
        self.projects.push(project);
    }
}
