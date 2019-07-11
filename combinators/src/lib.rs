/// Returns the suffix from file name.
pub fn suffix(file_name: &str) -> Option<&str> {
    match file_name.rfind(".") {
        Some(index) => Some(&file_name[index + 1..]),
        None => None,
    }
}

/// Returns the base name from path.
pub fn base_name(path: &str) -> Option<&str> {
    let index = path.rfind("/")?;
    Some(&path[index + 1..])
}

/// Demonstrates combinator usage.
pub fn path_suffix(path: &str) -> Option<&str> {
    base_name(path).and_then(suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_correct_suffix() {
        let fname = "foo.txt";
        let want = "txt";
        let got = suffix(fname).expect("failed to get existing suffix");
        assert_eq!(want, got);
    }

    #[test]
    #[should_panic]
    fn gets_none_when_no_file_suffix() {
        suffix("foo").expect("false positive");
    }

    #[test]
    fn gets_correct_base_name_from_path() {
        let path = "/path/to/foo.txt";
        let want = "foo.txt";
        let got = base_name(path).expect("failed to get filename from path");
        assert_eq!(want, got);
    }

    #[test]
    fn gets_path_suffix() {
        let path = "/path/to/foo.txt";
        let want = "txt";
        let got = path_suffix(path).expect("failed to get suffix");
        assert_eq!(want, got);
    }
}
