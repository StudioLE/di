use std::path::{Component, Path, PathBuf};

/// Normalize a path by resolving `.` and `..` components lexically.
///
/// This performs pure string manipulation without touching the filesystem.
/// It does NOT resolve symlinks or verify path existence.
#[must_use]
fn normalize_path(path: &Path) -> PathBuf {
    let mut result = PathBuf::new();
    for component in path.components() {
        match component {
            Component::ParentDir => {
                result.pop();
            }
            Component::CurDir => {}
            _ => result.push(component),
        }
    }
    result
}

/// Check if a path is safely within a base directory.
///
/// For existing paths, uses `canonicalize()` for accurate symlink resolution.
/// For non-existent paths, falls back to lexical normalization.
#[must_use]
pub fn is_path_within(path: &Path, base: &Path) -> bool {
    // For paths that exist, use canonicalize for accurate checking (resolves symlinks)
    if path.exists()
        && let (Ok(canonical_path), Ok(canonical_base)) = (path.canonicalize(), base.canonicalize())
    {
        return canonical_path.starts_with(&canonical_base);
    }

    // For non-existent paths or when canonicalize fails, use lexical normalization
    let normalized_path = normalize_path(path);
    let normalized_base = normalize_path(base);
    normalized_path.starts_with(&normalized_base)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_path_removes_current_dir() {
        let path = PathBuf::from("/foo/./bar/./baz");
        assert_eq!(normalize_path(&path), PathBuf::from("/foo/bar/baz"));
    }

    #[test]
    fn normalize_path_resolves_parent_dir() {
        let path = PathBuf::from("/foo/bar/../baz");
        assert_eq!(normalize_path(&path), PathBuf::from("/foo/baz"));
    }

    #[test]
    fn normalize_path_handles_multiple_parent_dirs() {
        let path = PathBuf::from("/foo/bar/baz/../../qux");
        assert_eq!(normalize_path(&path), PathBuf::from("/foo/qux"));
    }

    #[test]
    fn normalize_path_handles_mixed_components() {
        let path = PathBuf::from("/foo/./bar/../baz/./qux/../file");
        assert_eq!(normalize_path(&path), PathBuf::from("/foo/baz/file"));
    }

    #[test]
    fn normalize_path_preserves_absolute_path() {
        let path = PathBuf::from("/foo/bar");
        assert_eq!(normalize_path(&path), PathBuf::from("/foo/bar"));
    }

    #[test]
    fn normalize_path_handles_relative_path() {
        let path = PathBuf::from("foo/bar/../baz");
        assert_eq!(normalize_path(&path), PathBuf::from("foo/baz"));
    }

    #[test]
    fn normalize_path_handles_parent_at_root() {
        // Going above root just stays at root (pops nothing)
        let path = PathBuf::from("/../foo");
        let normalized = normalize_path(&path);
        assert_eq!(normalized, PathBuf::from("/foo"));
    }

    #[test]
    fn is_path_within_accepts_direct_child() {
        let path = PathBuf::from("/test/cache/file.txt");
        assert!(is_path_within(&path, &test_base()));
    }

    #[test]
    fn is_path_within_accepts_nested_child() {
        let path = PathBuf::from("/test/cache/foo/bar/file.txt");
        assert!(is_path_within(&path, &test_base()));
    }

    #[test]
    fn is_path_within_rejects_sibling() {
        let path = PathBuf::from("/test/other/file.txt");
        assert!(!is_path_within(&path, &test_base()));
    }

    #[test]
    fn is_path_within_rejects_traversal_escape() {
        let path = PathBuf::from("/test/cache/../other/file.txt");
        assert!(!is_path_within(&path, &test_base()));
    }

    #[test]
    fn is_path_within_rejects_deep_traversal_escape() {
        let path = PathBuf::from("/test/cache/foo/bar/../../../other/file.txt");
        assert!(!is_path_within(&path, &test_base()));
    }

    #[test]
    fn is_path_within_accepts_path_with_dots_resolved() {
        let path = PathBuf::from("/test/cache/foo/../bar/file.txt");
        assert!(is_path_within(&path, &test_base()));
    }

    #[test]
    fn is_path_within_handles_base_as_self() {
        let path = PathBuf::from("/test/cache");
        assert!(is_path_within(&path, &test_base()));
    }

    fn test_base() -> PathBuf {
        PathBuf::from("/test/cache")
    }
}
