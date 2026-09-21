// Utility helper functions

use std::path::Path;
use walkdir::WalkDir;

pub fn detect_language_for_path(path: &str) -> String {
    let p = Path::new(path);
    if p.join("Cargo.toml").exists() {
        "Rust".to_string()
    } else if p.join("package.json").exists() {
        "JavaScript".to_string()
    } else if p.join("pyproject.toml").exists() {
        "Python".to_string()
    } else if p.join("go.mod").exists() {
        "Go".to_string()
    } else if p.join("Makefile").exists()
        || p.join("CMakeLists.txt").exists()
        || p.join("configure.ac").exists()
    {
        "C/C++".to_string()
    } else if p.join("pom.xml").exists() || p.join("build.gradle").exists() {
        "Java".to_string()
    } else if p.join("composer.json").exists() {
        "PHP".to_string()
    } else if p.join("Gemfile").exists() {
        "Ruby".to_string()
    } else if p.join("Package.swift").exists() {
        "Swift".to_string()
    } else if p.join("build.gradle.kts").exists() {
        "Kotlin".to_string()
    } else if p.join("build.sbt").exists() {
        "Scala".to_string()
    } else if p.join("stack.yaml").exists() {
        "Haskell".to_string()
    } else if p.join("mix.exs").exists() {
        "Elixir".to_string()
    } else {
        "Unknown".to_string()
    }
}

pub fn calculate_dir_size(path: &str) -> u64 {
    use std::fs;
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| fs::metadata(e.path()).ok())
        .map(|m| m.len())
        .sum()
}
