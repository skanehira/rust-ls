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

fn should_skip(path: &str) -> bool {
    let pattern = Regex::new(r"\./(.git|target).*").unwrap();
    pattern.is_match(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execlude_word() {
        for word in vec!["./.git", "./target"] {
            assert!(should_skip(word));
        }
    }

    #[test]
    fn test_list() -> io::Result<()> {
        let dir = Path::new(".");
        let want = vec![
            "./Cargo.toml",
            "./Cargo.lock",
            "./src/lib.rs",
            "./src/main.rs",
        ];
        assert_eq!(want, visit_dirs(dir)?);
        Ok(())
    }
}

