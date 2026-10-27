//! Post-generation edits applied to Progenitor's `lib.rs` output.

const MODULE: &str = "mod progenitor_client;";
const ALLOW: &str = "#[allow(clippy::result_large_err)]\n";

/// Allow `clippy::result_large_err` on the vendored Progenitor support module.
///
/// Only the first module declaration is patched; input that is already patched
/// is returned unchanged so the step is idempotent.
pub fn allow_large_err(source: &str) -> Result<String, String> {
    let index = source
        .find(MODULE)
        .ok_or_else(|| format!("generated source has no `{MODULE}` declaration"))?;
    if source[..index].ends_with(ALLOW) {
        return Ok(source.to_owned());
    }
    let mut patched = String::with_capacity(source.len() + ALLOW.len());
    patched.push_str(&source[..index]);
    patched.push_str(ALLOW);
    patched.push_str(&source[index..]);
    Ok(patched)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inserts_allow_before_first_declaration_only() {
        let source = "//! doc\nmod progenitor_client;\nmod progenitor_client;\n";
        assert_eq!(
            allow_large_err(source).unwrap(),
            "//! doc\n#[allow(clippy::result_large_err)]\nmod progenitor_client;\nmod progenitor_client;\n"
        );
    }

    #[test]
    fn patching_is_idempotent() {
        let once = allow_large_err("mod progenitor_client;\n").unwrap();
        assert_eq!(allow_large_err(&once).unwrap(), once);
    }

    #[test]
    fn missing_declaration_is_an_error() {
        assert!(allow_large_err("pub mod types {}\n").is_err());
    }
}
