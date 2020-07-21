pub mod snap;
pub mod debian;
pub mod flatpak;
pub mod manifest;

pub fn has_type(type_name: String) -> bool {
    if type_name == "debian" {
        return true;
    }
    if type_name == "flatpak" {
        return true;
    }
    if type_name == "snap" {
        return true;
    }
    return false;
}

// Determines if the filename is a potential manifest
// of any supported build system.
pub fn get_type(ctx: &mut crate::execution_context::ExecutionContext) -> i32 {
    // FIXME the filename could match multiple build systems. How should we handle
    // that? If we try parsing a format and it fails, we might want to got back
    // and try parsing using other formats?
    if crate::manifests::debian::file_path_matches(&ctx.source_filename) {
        ctx.source_type = "debian".to_string();
    }

    if crate::manifests::snap::file_path_matches(&ctx.source_filename) {
        ctx.source_type = "snap".to_string();
    }

    if crate::manifests::flatpak::file_path_matches(&ctx.source_filename) {
        ctx.source_type = "flatpak".to_string();
    }

    return 0;
}

// Get the top-level build system for the project.
pub fn get_build_system(
    ctx: &mut crate::execution_context::ExecutionContext
) -> crate::manifests::manifest::BuildSystem {
    if ctx.source_filename.ends_with("meson_options.txt") {
        return crate::manifests::manifest::BuildSystem::meson;
    }
    if ctx.source_filename.ends_with("control") {
        // return crate::manifests::manifest::BuildSystem::debian;
    }
    if ctx.source_filename.ends_with("package.json") {
        return crate::manifests::manifest::BuildSystem::npm;
    }
    if ctx.source_filename.ends_with("Gemfile") {
        // return crate::manifests::manifest::BuildSystem::ruby;
    }
    if ctx.source_filename.ends_with("requirements.txt") {
        // We could also default to pip3...
        return crate::manifests::manifest::BuildSystem::pip3;
    }
    if ctx.source_filename.ends_with(".spec") {
        // return crate::manifests::manifest::BuildSystem::fedora;
    }
    if ctx.source_filename.ends_with("Makefile") {
        return crate::manifests::manifest::BuildSystem::make;
    }
    return crate::manifests::manifest::DEFAULT_BUILD_SYSTEM;
}

pub fn parse(ctx: &mut crate::execution_context::ExecutionContext) -> i32 {
    if ctx.source_type == "debian" {
        ctx.manifest = crate::manifests::debian::parse(&ctx.content);
    }

    if ctx.source_type == "snap" {
        ctx.manifest = crate::manifests::snap::parse(&ctx.content);
    }

    if ctx.source_type == "flatpak" {
        ctx.manifest = crate::manifests::flatpak::parse(&ctx.content);
    }

    return 1;
}

pub fn dump(ctx: &mut crate::execution_context::ExecutionContext) -> i32 {
    if ctx.destination_type == "debian" {
        crate::manifests::debian::dump(&ctx.manifest);
        return 0;
    }

    if ctx.destination_type == "snap" {
        crate::manifests::snap::dump(&ctx.manifest);
        return 0;
    }

    if ctx.destination_type == "flatpak" {
        crate::manifests::flatpak::dump(&ctx.manifest);
        return 0;
    }

    panic!("Invalid destination type {}.", ctx.destination_type);
}
