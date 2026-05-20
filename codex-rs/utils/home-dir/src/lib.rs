use dirs::home_dir;
use std::path::PathBuf;

pub const CODEX_UNLEASHED_HOME_ENV_VAR: &str = "CODEX_UNLEASHED_HOME";
pub const LEGACY_CODEX_HOME_ENV_VAR: &str = "CODEX_HOME";
pub const DEFAULT_CODEX_UNLEASHED_HOME_DIR: &str = ".codex-unleashed";

/// Returns the path to the Codex Unleashed configuration directory, which can
/// be specified by the `CODEX_UNLEASHED_HOME` environment variable. If not set,
/// defaults to `~/.codex-unleashed`.
///
/// - If `CODEX_UNLEASHED_HOME` is set, the value must exist and be a
///   directory. The value will be canonicalized and this function will Err
///   otherwise.
/// - If `CODEX_UNLEASHED_HOME` is not set, `CODEX_HOME` is honored as a legacy
///   override for tests and explicit migrations.
/// - If neither environment variable is set, this function does not verify that
///   the directory exists.
pub fn find_codex_home() -> std::io::Result<PathBuf> {
    let codex_unleashed_home_env = std::env::var(CODEX_UNLEASHED_HOME_ENV_VAR)
        .ok()
        .filter(|val| !val.is_empty());
    let legacy_codex_home_env = std::env::var(LEGACY_CODEX_HOME_ENV_VAR)
        .ok()
        .filter(|val| !val.is_empty());
    find_codex_home_from_env(
        codex_unleashed_home_env.as_deref(),
        legacy_codex_home_env.as_deref(),
    )
}

fn find_codex_home_from_env(
    codex_unleashed_home_env: Option<&str>,
    legacy_codex_home_env: Option<&str>,
) -> std::io::Result<PathBuf> {
    if let Some(val) = codex_unleashed_home_env {
        return resolve_env_home(CODEX_UNLEASHED_HOME_ENV_VAR, val);
    }
    if let Some(val) = legacy_codex_home_env {
        return resolve_env_home(LEGACY_CODEX_HOME_ENV_VAR, val);
    }

    let mut p = home_dir().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find home directory",
        )
    })?;
    p.push(DEFAULT_CODEX_UNLEASHED_HOME_DIR);
    Ok(p)
}

fn resolve_env_home(env_name: &str, val: &str) -> std::io::Result<PathBuf> {
    let path = PathBuf::from(val);
    let metadata = std::fs::metadata(&path).map_err(|err| match err.kind() {
        std::io::ErrorKind::NotFound => std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("{env_name} points to {val:?}, but that path does not exist"),
        ),
        _ => std::io::Error::new(
            err.kind(),
            format!("failed to read {env_name} {val:?}: {err}"),
        ),
    })?;

    if !metadata.is_dir() {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("{env_name} points to {val:?}, but that path is not a directory"),
        ))
    } else {
        path.canonicalize().map_err(|err| {
            std::io::Error::new(
                err.kind(),
                format!("failed to canonicalize {env_name} {val:?}: {err}"),
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::CODEX_UNLEASHED_HOME_ENV_VAR;
    use super::DEFAULT_CODEX_UNLEASHED_HOME_DIR;
    use super::LEGACY_CODEX_HOME_ENV_VAR;
    use super::find_codex_home_from_env;
    use dirs::home_dir;
    use pretty_assertions::assert_eq;
    use std::fs;
    use std::io::ErrorKind;
    use tempfile::TempDir;

    #[test]
    fn find_codex_home_env_missing_path_is_fatal() {
        let temp_home = TempDir::new().expect("temp home");
        let missing = temp_home.path().join("missing-codex-home");
        let missing_str = missing
            .to_str()
            .expect("missing codex home path should be valid utf-8");

        let err = find_codex_home_from_env(Some(missing_str), None)
            .expect_err("missing CODEX_UNLEASHED_HOME");
        assert_eq!(err.kind(), ErrorKind::NotFound);
        assert!(
            err.to_string().contains(CODEX_UNLEASHED_HOME_ENV_VAR),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn find_codex_home_env_file_path_is_fatal() {
        let temp_home = TempDir::new().expect("temp home");
        let file_path = temp_home.path().join("codex-home.txt");
        fs::write(&file_path, "not a directory").expect("write temp file");
        let file_str = file_path
            .to_str()
            .expect("file codex home path should be valid utf-8");

        let err =
            find_codex_home_from_env(Some(file_str), None).expect_err("file CODEX_UNLEASHED_HOME");
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert!(
            err.to_string().contains("not a directory"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn find_codex_home_env_valid_directory_canonicalizes() {
        let temp_home = TempDir::new().expect("temp home");
        let temp_str = temp_home
            .path()
            .to_str()
            .expect("temp codex home path should be valid utf-8");

        let resolved =
            find_codex_home_from_env(Some(temp_str), None).expect("valid CODEX_UNLEASHED_HOME");
        let expected = temp_home
            .path()
            .canonicalize()
            .expect("canonicalize temp home");
        assert_eq!(resolved, expected);
    }

    #[test]
    fn find_codex_home_prefers_unleashed_home_over_legacy_codex_home() {
        let unleashed_home = TempDir::new().expect("unleashed home");
        let legacy_home = TempDir::new().expect("legacy codex home");
        let unleashed_str = unleashed_home
            .path()
            .to_str()
            .expect("unleashed codex home path should be valid utf-8");
        let legacy_str = legacy_home
            .path()
            .to_str()
            .expect("legacy codex home path should be valid utf-8");

        let resolved = find_codex_home_from_env(Some(unleashed_str), Some(legacy_str))
            .expect("valid CODEX_UNLEASHED_HOME");
        let expected = unleashed_home
            .path()
            .canonicalize()
            .expect("canonicalize unleashed home");
        assert_eq!(resolved, expected);
    }

    #[test]
    fn find_codex_home_uses_legacy_codex_home_when_unleashed_home_is_unset() {
        let legacy_home = TempDir::new().expect("legacy codex home");
        let legacy_str = legacy_home
            .path()
            .to_str()
            .expect("legacy codex home path should be valid utf-8");

        let resolved =
            find_codex_home_from_env(None, Some(legacy_str)).expect("valid legacy CODEX_HOME");
        let expected = legacy_home
            .path()
            .canonicalize()
            .expect("canonicalize legacy home");
        assert_eq!(resolved, expected);
    }

    #[test]
    fn find_codex_home_without_env_uses_unleashed_default_home_dir() {
        let resolved = find_codex_home_from_env(None, None).expect("default CODEX_UNLEASHED_HOME");
        let mut expected = home_dir().expect("home dir");
        expected.push(DEFAULT_CODEX_UNLEASHED_HOME_DIR);
        assert_eq!(resolved, expected);
    }

    #[test]
    fn env_var_constants_document_supported_override_names() {
        assert_eq!(CODEX_UNLEASHED_HOME_ENV_VAR, "CODEX_UNLEASHED_HOME");
        assert_eq!(LEGACY_CODEX_HOME_ENV_VAR, "CODEX_HOME");
    }
}
