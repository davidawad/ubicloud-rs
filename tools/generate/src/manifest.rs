//! Minimal, formatting-preserving edits of string keys in `Cargo.toml`.
//!
//! Only `key = "value"` lines inside a named table are supported, which covers
//! `[package]` and `[package.metadata.upstream-spec]`.

pub const PACKAGE: &str = "package";
pub const UPSTREAM_SPEC: &str = "package.metadata.upstream-spec";

/// Return the line range (exclusive of the header) of table `section`.
fn section_lines(lines: &[&str], section: &str) -> Result<(usize, usize), String> {
    let header = format!("[{section}]");
    let start = lines
        .iter()
        .position(|line| line.trim() == header)
        .ok_or_else(|| format!("Cargo.toml has no {header} table"))?
        + 1;
    let end = lines[start..]
        .iter()
        .position(|line| line.trim_start().starts_with('['))
        .map_or(lines.len(), |offset| start + offset);
    Ok((start, end))
}

/// Return the quoted value if `line` assigns `key`.
fn assigned_value<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let rest = line.trim_start().strip_prefix(key)?.trim_start();
    let value = rest.strip_prefix('=')?.trim();
    value.strip_prefix('"')?.split('"').next()
}

/// Read string `key` from table `section`.
pub fn get(manifest: &str, section: &str, key: &str) -> Result<String, String> {
    let lines: Vec<&str> = manifest.lines().collect();
    let (start, end) = section_lines(&lines, section)?;
    lines[start..end]
        .iter()
        .find_map(|line| assigned_value(line, key))
        .map(str::to_owned)
        .ok_or_else(|| format!("[{section}] has no string `{key}`"))
}

/// Replace string `key` in table `section` with `value`.
pub fn set(manifest: &str, section: &str, key: &str, value: &str) -> Result<String, String> {
    if value.contains(['"', '\\', '\n']) {
        return Err(format!("refusing to write unescaped value {value:?}"));
    }
    let lines: Vec<&str> = manifest.lines().collect();
    let (start, end) = section_lines(&lines, section)?;
    let index = (start..end)
        .find(|&index| assigned_value(lines[index], key).is_some())
        .ok_or_else(|| format!("[{section}] has no string `{key}`"))?;
    let replacement = format!("{key} = \"{value}\"");
    let mut updated: Vec<&str> = lines.clone();
    updated[index] = &replacement;
    let mut output = updated.join("\n");
    if manifest.ends_with('\n') {
        output.push('\n');
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MANIFEST: &str = r#"[package]
name = "ubicloud-rs"
version = "0.1.0+api.abc"
rust-version = "1.88"

[dependencies]
version = "not-the-package"

[package.metadata.upstream-spec]
revision = "00f2"
sha256 = "52d9"
"#;

    #[test]
    fn reads_keys_from_the_requested_table() {
        assert_eq!(get(MANIFEST, PACKAGE, "version").unwrap(), "0.1.0+api.abc");
        assert_eq!(get(MANIFEST, UPSTREAM_SPEC, "sha256").unwrap(), "52d9");
        assert!(get(MANIFEST, PACKAGE, "sha256").is_err());
        assert!(get(MANIFEST, "workspace", "members").is_err());
    }

    #[test]
    fn version_prefix_does_not_match_other_keys() {
        let manifest = "[package]\nversions = \"x\"\nversion = \"1.2.3\"\n";
        assert_eq!(get(manifest, PACKAGE, "version").unwrap(), "1.2.3");
    }

    #[test]
    fn set_replaces_only_the_target_line() {
        let updated = set(MANIFEST, PACKAGE, "version", "0.2.0+api.def").unwrap();
        assert_eq!(
            updated,
            MANIFEST.replace("version = \"0.1.0+api.abc\"", "version = \"0.2.0+api.def\"")
        );
        let updated = set(&updated, UPSTREAM_SPEC, "revision", "1234").unwrap();
        assert_eq!(get(&updated, UPSTREAM_SPEC, "revision").unwrap(), "1234");
        assert_eq!(
            get(&updated, "dependencies", "version").unwrap(),
            "not-the-package"
        );
    }

    #[test]
    fn set_rejects_values_needing_escapes() {
        assert!(set(MANIFEST, PACKAGE, "version", "1\"").is_err());
    }
}
