use regex::Regex;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn start(formula: &str) {
    println!("starting {} formula", formula);
}

pub fn find_formula_plist_file(formula: &str) -> PathBuf {
    let mut path: PathBuf = PathBuf::new();
    let re = Regex::new(r"homebrew(.mxcl)?\.([\w+-.@]+)(\.plist|\.service)?\z").unwrap();
    for entry in WalkDir::new("/usr/local/Cellar")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let p = entry.path();
        if re.is_match(p.to_str().unwrap()) && p.to_str().unwrap().contains(&formula) {
            // println!("{}", entry.path().display());
            path = p.to_path_buf();
            break;
        }
    }

    path
}
