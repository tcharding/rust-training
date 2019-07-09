/// Returns the file extension from fname.
pub fn extension(fname: &str) -> Option<&str> {
    let index = fname.rfind(".")?;
    Some(&fname[index + 1..])
}

/// Returns the base name from path.
pub fn base_name(path: &str) -> Option<&str> {
    let index = path.rfind("/")?;
    Some(&path[index + 1..])
}

/// Demonstrates combinator usage.
pub fn file_path_extension(path: &str) -> Option<&str> {
    base_name(path).and_then(extension)
}

/// Combinator that calls f if option contains a value.
pub fn and_then<F, T, A>(option: Option<T>, f: F) -> Option<A>
where
    F: FnOnce(T) -> Option<A>,
{
    let value = option?;
    f(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_correct_extension() {
        let fname = "foo.txt";
        let want = "txt";
        let got = extension(fname).expect("failed to get existing extension");
        assert_eq!(want, got);
    }

    #[test]
    #[should_panic]
    fn gets_none_when_no_file_extension() {
        extension("foo").expect("false positive");
    }

    #[test]
    fn gets_correct_base_name_from_path() {
        let path = "/path/to/foo.txt";
        let want = "foo.txt";
        let got = base_name(path).expect("failed to get filename from path");
        assert_eq!(want, got);
    }

    #[test]
    fn gets_file_path_extension() {
        let path = "/path/to/foo.txt";
        let want = "txt";
        let got = file_path_extension(path).expect("failde to get extension");
        assert_eq!(want, got);
    }
}
