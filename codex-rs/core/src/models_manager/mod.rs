pub mod cache;
pub mod collaboration_mode_presets;
pub mod manager;
pub mod model_info;
pub mod model_presets;

// Keep 0.87-era and dev builds eligible for the current OpenAI `/models` catalog
// without changing the user-visible CLI version.
const MODEL_DISCOVERY_CLIENT_VERSION_FLOOR: &str = "0.98.0";

/// Return the whole client version used for model discovery, with a compatibility floor.
pub fn client_version_to_whole() -> String {
    let package_version = format!(
        "{}.{}.{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH")
    );
    client_version_for_model_discovery(&package_version)
}

fn client_version_for_model_discovery(package_version: &str) -> String {
    if semver_triplet_lt(package_version, MODEL_DISCOVERY_CLIENT_VERSION_FLOOR) {
        MODEL_DISCOVERY_CLIENT_VERSION_FLOOR.to_string()
    } else {
        package_version.to_string()
    }
}

fn semver_triplet_lt(left: &str, right: &str) -> bool {
    let left = parse_semver_triplet(left);
    let right = parse_semver_triplet(right);
    left < right
}

fn parse_semver_triplet(version: &str) -> [u64; 3] {
    let mut parts = [0, 0, 0];
    for (index, part) in version.split('.').take(3).enumerate() {
        parts[index] = part.parse().unwrap_or(0);
    }
    parts
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn model_discovery_client_version_uses_compatibility_floor() {
        assert_eq!(
            client_version_for_model_discovery("0.87.0"),
            MODEL_DISCOVERY_CLIENT_VERSION_FLOOR
        );
        assert_eq!(
            client_version_for_model_discovery("0.0.0"),
            MODEL_DISCOVERY_CLIENT_VERSION_FLOOR
        );
        assert_eq!(client_version_for_model_discovery("0.98.0"), "0.98.0");
        assert_eq!(client_version_for_model_discovery("0.99.0"), "0.99.0");
    }
}
