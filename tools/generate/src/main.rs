//! Generation and release helpers driven by `scripts/generate-client.sh` and
//! `scripts/regen`. Each subcommand prints its result on stdout and reports
//! errors on stderr with a nonzero exit status.

mod manifest;
mod patch;
mod spec;
mod version;

use std::io::Read;
use std::process::ExitCode;

use version::{Bump, Version};

const USAGE: &str = "\
usage: generate <command> [args]

commands:
  prepare-spec <source.yml> <destination.yml>
      rewrite `default` responses as 4XX/5XX for Progenitor
  patch-generated <mod.rs>
      allow clippy::result_large_err on the progenitor_client module
  package-version <Cargo.toml>
      print the [package] version without build metadata
  set-package-version <Cargo.toml> <version>
      replace the [package] version
  set-package-license <Cargo.toml> <license>
      set the [package] license
  spec-metadata <Cargo.toml> <key>
      print a [package.metadata.upstream-spec] value
  set-spec-metadata <Cargo.toml> <key>=<value>...
      replace [package.metadata.upstream-spec] values
  latest-tag
      print the highest vMAJOR.MINOR.PATCH[+build] tag read from stdin
  tag-version <tag>
      print the MAJOR.MINOR.PATCH of a release tag
  semver-release-type <version>
      print the cargo-semver-checks --release-type that rejects breaking changes
  next-version <version> <patch|minor|major>
      print the bumped version
  release-tag <version> <revision>
      print v<version>+api.<revision>";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(output) => {
            if !output.is_empty() {
                println!("{output}");
            }
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("generate: {message}");
            ExitCode::FAILURE
        }
    }
}

fn read(path: &str) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|error| format!("cannot read {path}: {error}"))
}

fn write(path: &str, contents: &str) -> Result<(), String> {
    std::fs::write(path, contents).map_err(|error| format!("cannot write {path}: {error}"))
}

fn run(args: &[String]) -> Result<String, String> {
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    match args[..] {
        ["prepare-spec", source, destination] => {
            write(destination, &spec::prepare(&read(source)?)?)?;
            Ok(String::new())
        }
        ["patch-generated", path] => {
            write(path, &patch::allow_large_err(&read(path)?)?)?;
            Ok(String::new())
        }
        ["package-version", path] => {
            let version = manifest::get(&read(path)?, manifest::PACKAGE, "version")?;
            Ok(Version::parse(&version)?.to_string())
        }
        ["set-package-version", path, version] => {
            Version::parse(version)?;
            let updated = manifest::set(&read(path)?, manifest::PACKAGE, "version", version)?;
            write(path, &updated)?;
            Ok(String::new())
        }
        ["set-package-license", path, license] => {
            let updated = manifest::set(&read(path)?, manifest::PACKAGE, "license", license)?;
            write(path, &updated)?;
            Ok(String::new())
        }
        ["spec-metadata", path, key] => manifest::get(&read(path)?, manifest::UPSTREAM_SPEC, key),
        ["set-spec-metadata", path, ref assignments @ ..] if !assignments.is_empty() => {
            let mut contents = read(path)?;
            for assignment in assignments {
                let (key, value) = assignment
                    .split_once('=')
                    .ok_or_else(|| format!("expected <key>=<value>, got {assignment:?}"))?;
                contents = manifest::set(&contents, manifest::UPSTREAM_SPEC, key, value)?;
            }
            write(path, &contents)?;
            Ok(String::new())
        }
        ["latest-tag"] => {
            let mut tags = String::new();
            std::io::stdin()
                .read_to_string(&mut tags)
                .map_err(|error| format!("cannot read tags from stdin: {error}"))?;
            Ok(version::latest_tag(tags.lines())
                .unwrap_or_default()
                .to_owned())
        }
        ["tag-version", tag] => Ok(Version::from_tag(tag)?.to_string()),
        ["semver-release-type", version] => Ok(Version::parse(version)?
            .non_breaking_release_type()
            .to_owned()),
        ["next-version", version, bump] => Ok(Version::parse(version)?
            .bump(Bump::parse(bump)?)
            .to_string()),
        ["release-tag", version, revision] => {
            version::release_tag(Version::parse(version)?, revision)
        }
        _ => Err(format!("invalid arguments {args:?}\n{USAGE}")),
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|arg| (*arg).to_owned()).collect()
    }

    #[test]
    fn version_commands_compose() {
        assert_eq!(
            run(&args(&["tag-version", "v0.1.0+api.abc"])).unwrap(),
            "0.1.0"
        );
        assert_eq!(
            run(&args(&["next-version", "0.1.0", "minor"])).unwrap(),
            "0.2.0"
        );
        assert_eq!(
            run(&args(&["semver-release-type", "0.1.0"])).unwrap(),
            "patch"
        );
        assert_eq!(
            run(&args(&["release-tag", "0.2.0", "abc123"])).unwrap(),
            "v0.2.0+api.abc123"
        );
    }

    #[test]
    fn package_version_is_the_release_tag_without_its_v_prefix() {
        let tag = run(&args(&["release-tag", "0.2.0", "abc123"])).unwrap();
        let package_version = tag.strip_prefix('v').unwrap();
        let path =
            std::env::temp_dir().join(format!("generate-tag-test-{}.toml", std::process::id()));
        let path_text = path.to_str().unwrap();
        std::fs::write(&path, "[package]\nversion = \"0.1.0\"\n").unwrap();

        run(&args(&["set-package-version", path_text, package_version])).unwrap();
        let manifest = std::fs::read_to_string(&path).unwrap();
        assert_eq!(manifest, "[package]\nversion = \"0.2.0+api.abc123\"\n");
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn unknown_commands_print_usage() {
        let error = run(&args(&["frobnicate"])).unwrap_err();
        assert!(error.contains("usage: generate"));
        assert!(run(&args(&["set-spec-metadata", "Cargo.toml"])).is_err());
    }

    #[test]
    fn manifest_commands_edit_files() {
        let path = std::env::temp_dir().join(format!("generate-test-{}.toml", std::process::id()));
        let path_text = path.to_str().unwrap();
        std::fs::write(
            &path,
            "[package]\nversion = \"0.1.0+api.a\"\nlicense = \"BSD-3-Clause\"\n\n[package.metadata.upstream-spec]\nsha256 = \"x\"\nrevision = \"a\"\n",
        )
        .unwrap();
        run(&args(&[
            "set-package-license",
            path_text,
            "GPL-3.0-or-later",
        ]))
        .unwrap();
        assert!(
            std::fs::read_to_string(&path)
                .unwrap()
                .contains("license = \"GPL-3.0-or-later\"")
        );
        assert_eq!(
            run(&args(&["package-version", path_text])).unwrap(),
            "0.1.0"
        );
        run(&args(&["set-package-version", path_text, "0.2.0+api.b"])).unwrap();
        run(&args(&[
            "set-spec-metadata",
            path_text,
            "sha256=y",
            "revision=b",
        ]))
        .unwrap();
        assert_eq!(
            run(&args(&["spec-metadata", path_text, "sha256"])).unwrap(),
            "y"
        );
        assert_eq!(
            run(&args(&["spec-metadata", path_text, "revision"])).unwrap(),
            "b"
        );
        assert_eq!(
            run(&args(&["package-version", path_text])).unwrap(),
            "0.2.0"
        );
        assert!(run(&args(&["set-package-version", path_text, "nope"])).is_err());
        std::fs::remove_file(path).unwrap();
    }
}
