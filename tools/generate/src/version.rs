//! Release version parsing and bump selection for scripts/regen.

use std::fmt;

/// A plain `MAJOR.MINOR.PATCH` release version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

/// The kind of release produced by a regeneration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bump {
    /// The spec changed but the generated client did not.
    Patch,
    /// Additive (semver-compatible) API change.
    Minor,
    /// cargo-semver-checks reported a breaking change.
    Major,
}

impl Bump {
    pub fn parse(text: &str) -> Result<Self, String> {
        match text {
            "patch" => Ok(Self::Patch),
            "minor" => Ok(Self::Minor),
            "major" => Ok(Self::Major),
            other => Err(format!(
                "unknown bump {other:?}; expected patch, minor or major"
            )),
        }
    }
}

impl Version {
    /// Parse `MAJOR.MINOR.PATCH`, ignoring a `+build` suffix.
    ///
    /// Pre-release versions are rejected: regen never produces them.
    pub fn parse(text: &str) -> Result<Self, String> {
        let core = text.split_once('+').map_or(text, |(core, _)| core);
        let parts: Vec<&str> = core.split('.').collect();
        let [major, minor, patch] = parts[..] else {
            return Err(format!("cannot parse {text:?} as MAJOR.MINOR.PATCH"));
        };
        let number = |part: &str| {
            let valid = !part.is_empty()
                && part.bytes().all(|byte| byte.is_ascii_digit())
                && (part == "0" || !part.starts_with('0'));
            valid
                .then(|| part.parse::<u64>().ok())
                .flatten()
                .ok_or_else(|| format!("cannot parse {text:?} as MAJOR.MINOR.PATCH"))
        };
        Ok(Self {
            major: number(major)?,
            minor: number(minor)?,
            patch: number(patch)?,
        })
    }

    /// Parse a release tag such as `v0.2.0+api.<revision>`.
    pub fn from_tag(tag: &str) -> Result<Self, String> {
        let version = tag
            .strip_prefix('v')
            .ok_or_else(|| format!("tag {tag:?} does not start with `v`"))?;
        Self::parse(version)
    }

    pub fn bump(self, bump: Bump) -> Self {
        match bump {
            Bump::Patch => Self {
                patch: self.patch + 1,
                ..self
            },
            Bump::Minor => Self {
                minor: self.minor + 1,
                patch: 0,
                ..self
            },
            Bump::Major => Self {
                major: self.major + 1,
                minor: 0,
                patch: 0,
            },
        }
    }

    /// The `cargo semver-checks --release-type` that fails exactly on breaking
    /// changes relative to this baseline.
    ///
    /// Cargo treats `0.y` minor bumps as breaking, so for pre-1.0 baselines a
    /// `patch` check is the one that rejects breaking changes while allowing
    /// additions.
    pub fn non_breaking_release_type(self) -> &'static str {
        if self.major == 0 { "patch" } else { "minor" }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Pick the highest release tag; tags that are not `vMAJOR.MINOR.PATCH[+build]`
/// are ignored.
pub fn latest_tag<'a>(tags: impl IntoIterator<Item = &'a str>) -> Option<&'a str> {
    tags.into_iter()
        .map(str::trim)
        .filter_map(|tag| Version::from_tag(tag).ok().map(|version| (version, tag)))
        .max()
        .map(|(_, tag)| tag)
}

/// The release tag for `version` generated from upstream `revision`.
pub fn release_tag(version: Version, revision: &str) -> Result<String, String> {
    let valid = !revision.is_empty()
        && revision
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-');
    if !valid {
        return Err(format!(
            "revision {revision:?} is not valid semver build metadata"
        ));
    }
    Ok(format!("v{version}+api.{revision}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn version(text: &str) -> Version {
        Version::parse(text).unwrap()
    }

    #[test]
    fn parses_versions_and_ignores_build_metadata() {
        assert_eq!(
            version("0.1.0"),
            Version {
                major: 0,
                minor: 1,
                patch: 0
            }
        );
        assert_eq!(version("1.20.3+api.00f2bfa"), version("1.20.3"));
        assert_eq!(
            Version::from_tag("v2.0.1+api.abc").unwrap(),
            version("2.0.1")
        );
    }

    #[test]
    fn rejects_malformed_versions() {
        for text in [
            "",
            "1.2",
            "1.2.3.4",
            "1.2.x",
            "01.2.3",
            "1.2.3-rc.1",
            "v1.2.3",
        ] {
            assert!(Version::parse(text).is_err(), "{text:?} should be rejected");
        }
        assert!(Version::from_tag("1.2.3").is_err());
    }

    #[test]
    fn bumps_follow_semver() {
        assert_eq!(version("0.1.4").bump(Bump::Patch), version("0.1.5"));
        assert_eq!(version("0.1.4").bump(Bump::Minor), version("0.2.0"));
        assert_eq!(version("0.1.4").bump(Bump::Major), version("1.0.0"));
        assert_eq!(version("1.9.2").bump(Bump::Minor), version("1.10.0"));
        assert!(Bump::parse("huge").is_err());
    }

    #[test]
    fn breaking_check_release_type_accounts_for_pre_1_0() {
        assert_eq!(version("0.9.0").non_breaking_release_type(), "patch");
        assert_eq!(version("1.0.0").non_breaking_release_type(), "minor");
    }

    #[test]
    fn latest_tag_orders_numerically_and_skips_other_tags() {
        let tags = [
            "v0.9.0+api.a",
            "v0.10.0+api.b",
            "nightly",
            "v0.2.0-rc.1",
            " v0.3.0 ",
        ];
        assert_eq!(latest_tag(tags), Some("v0.10.0+api.b"));
        assert_eq!(latest_tag(["other"]), None);
    }

    #[test]
    fn release_tags_carry_the_api_revision() {
        assert_eq!(
            release_tag(version("0.2.0"), "00f2bfa6").unwrap(),
            "v0.2.0+api.00f2bfa6"
        );
        assert!(release_tag(version("0.2.0"), "bad/rev").is_err());
        assert!(release_tag(version("0.2.0"), "").is_err());
    }
}
