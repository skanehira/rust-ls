use regex::Regex;
use std::fs;
use std::io;
use std::path::Path;

pub fn visit_dirs(dir: &Path) -> io::Result<Vec<String>> {
    let mut paths = vec![];
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let target = path.to_string_lossy().to_string();

            if should_skip(&target) {
                continue;
            }
            if path.is_dir() {
                let children = visit_dirs(&path)?;
                for c in children {
                    paths.push(c);
                }
            } else {
                paths.push(target);
            }
        }
    }
    Ok(paths)
}

pub fn current_dir(dir: &Path) -> io::Result<Vec<String>> {
    let mut entries = vec![];

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if should_skip(&entry.path().to_string_lossy()) {
            continue;
        }
        entries.push(entry.file_name().to_string_lossy().to_string());
    }
    entries.sort();

    Ok(entries)
}

fn should_skip(path: &str) -> bool {
    let pattern = Regex::new(r"\./.git.*").unwrap();
    pattern.is_match(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_execlude_word() {
        assert!(should_skip("./.git"));
        assert_ne!(should_skip("./.target"), true);
    }

    #[test]
    fn test_current_dir() -> io::Result<()> {
        let dir = Path::new(".");
        let want = vec!["Cargo.lock", "Cargo.toml", "LICENSE", "src", "target"];
        assert_eq!(want, current_dir(dir)?);
        Ok(())
    }
}
