use regex::Regex;
use std::fs;
use std::io;
use std::path::Path;

// -r 時の出力用の構造体
pub struct RecursivelyOutput {
    pub dir: Option<String>,
    pub entries: Vec<String>,
}

pub fn visit_dirs(dir: &Path) -> io::Result<Vec<RecursivelyOutput>> {
    let mut output = vec![];
    let mut chidlren_output = vec![];

    if dir.is_dir() {
        let mut current_dir_output = RecursivelyOutput {
            dir: Some(dir.to_string_lossy().to_string()),
            entries: vec![],
        };

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let target = path.to_string_lossy().to_string();
            if should_skip(&target) {
                continue;
            }

            // 引数で受け取ったディレクトリ内のファイルorディレクトリを追加
            current_dir_output
                .entries
                .push(entry.file_name().to_string_lossy().to_string());

            // 子ディレクトリがある場合は、さらにその中を探索する
            if path.is_dir() {
                let output = visit_dirs(&path)?;
                chidlren_output.extend(output);
            }
        }
        output.push(current_dir_output);
        output.extend(chidlren_output);
    }
    Ok(output)
}

pub fn print_recursively_output(output: Vec<RecursivelyOutput>) {
    for out in output {
        if out.dir.is_some() {
            println!("{}", out.dir.unwrap());
        }
        for e in out.entries {
            println!("{}", e);
        }
        println!("");
    }
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
    let pattern = Regex::new(r"\./(.git|target).*").unwrap();
    pattern.is_match(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_should_skip() {
        let skip_dirs = vec!["./.git", "./target"];
        for dir in skip_dirs {
            assert!(should_skip(dir));
        }
    }

    #[test]
    fn test_current_dir() -> io::Result<()> {
        let dir = Path::new(".");
        let want = vec!["Cargo.lock", "Cargo.toml", "LICENSE", "src"];
        assert_eq!(want, current_dir(dir)?);
        Ok(())
    }
}
