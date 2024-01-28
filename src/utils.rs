use std::path::PathBuf;

pub fn relative_to(path: &PathBuf, base: &PathBuf) -> PathBuf {
    path.strip_prefix(base)
        .unwrap_or_else(|_| panic!("Base is not a prefix of path"))
        .to_path_buf()
}

pub fn all_relative_to(paths: &[PathBuf], base: &PathBuf) -> Vec<PathBuf> {
    paths
        .iter()
        .map(|path| relative_to(path, base))
        .collect::<Vec<PathBuf>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_to() {
        let path = PathBuf::from("/a/b/c");
        let base = PathBuf::from("/a");
        let relative = relative_to(&path, &base);
        assert_eq!(relative, PathBuf::from("b/c"));
    }

    #[test]
    fn test_all_relative_to() {
        let paths = vec![
            PathBuf::from("/a/b/c"),
            PathBuf::from("/a/b/d"),
            PathBuf::from("/a/b/e"),
        ];
        let base = PathBuf::from("/a");
        let relative = all_relative_to(&paths, &base);
        assert_eq!(
            relative,
            vec![
                PathBuf::from("b/c"),
                PathBuf::from("b/d"),
                PathBuf::from("b/e")
            ]
        );
    }
}