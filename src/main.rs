use clap::Parser;
use regex::{RegexBuilder};
use std::{
    env,
    path::{Component, Path, PathBuf},
};
use walkdir::WalkDir;

fn wildcard_to_regex(pattern: &str) -> String {
    let escaped = regex::escape(pattern);
    let replaced = escaped.replace(r"\*", ".*").replace(r"\?", ".");
    format!("^{}$", replaced)
}

fn match_path_components(components: &[&str], base: &Path) -> Option<PathBuf> {
    if components.is_empty() {
        return base.is_dir().then(|| base.to_path_buf());
    }

    let current_pattern = components[0];
    let regex_str = wildcard_to_regex(current_pattern);
    let regex = RegexBuilder::new(&regex_str)
        .case_insensitive(true)
        .build()
        .ok()?;

    WalkDir::new(base)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
        .find_map(|entry| {
            let dir_name = entry.file_name().to_str()?;
            if regex.is_match(dir_name) {
                let new_base = entry.path();
                if components.len() == 1 {
                    Some(new_base.to_path_buf())
                } else {
                    match_path_components(&components[1..], new_base)
                }
            } else {
                None
            }
        })
}

fn resolve_path(pattern: &str) -> Option<PathBuf> {
    let normalized = normalize_path(pattern);
    
    if normalized.is_dir() {
        return Some(normalized);
    }

    let components: Vec<_> = Path::new(pattern)
        .components()
        .filter_map(|c| match c {
            Component::Normal(s) => s.to_str(),
            _ => None,
        })
        .collect();

    if components.is_empty() {
        return None;
    }

    let base = env::current_dir().ok()?;
    match_path_components(&components, &base)
}

fn normalize_path(input: &str) -> PathBuf {
    let trimmed = input.trim_matches(|c| c == '"' || c == '\'')
        .replace("/", "\\");
    
    let path = PathBuf::from(trimmed);
    if path.is_absolute() {
        path
    } else {
        env::current_dir().unwrap().join(path)
    }
}

#[derive(Parser)]
#[command(
    author = StarWindv(星灿长风v),
    version = 0.0.1,
    about = "Windows 通配符目录切换工具",
    long_about = None
)]
struct Args {
    path_pattern: String,
}


fn main() {
    let args = Args::parse();
    
    match resolve_path(&args.path_pattern) {
        Some(path) => println!("{}", path.display()),
        None => {
            eprintln!("错误: 目录不存在或无法访问");
            std::process::exit(1);
        }
    }
}
