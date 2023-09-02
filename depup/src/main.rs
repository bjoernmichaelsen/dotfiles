use hyper_tls::HttpsConnector;
use hyper::Client;
use hyper::Request;
use itertools::Itertools;
use tokio::fs;
use std::collections::HashMap;
use std::env;
use std::str;
use std::str::FromStr;
use serde_json::Value;
use semver::Version;
use configparser::ini::Ini;

fn get_tag_name(entry: &Value) -> Option<&str> {
    Some(entry.as_object()?["name"].as_str()?)
}
async fn parse_tags_json(json_to_parse: &str) -> Option<Vec<String>> {
    let v : Value = serde_json::from_str(json_to_parse).ok()?;
    let entries = v.as_array()?;
    let str_res = entries
        .iter()
        .filter_map(|e| {get_tag_name(e)});
    Some(str_res
        .into_iter()
        .map(str::to_owned)
        .collect_vec())
}
async fn find_latest_version(versions: Vec<&str>) -> Option<Version> {
    versions
        .into_iter()
        .filter_map(|v| { Version::parse(v).ok()})
        .max()
}
async fn get_repo_tags_json(repo: &str, token: &str) -> Option<String> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let url = format!("https://api.github.com/repos/{}/tags", repo);
    let req = Request::builder()
        .uri(url)
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", token))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "depup/1.0")
        .body(hyper::Body::empty())
        .unwrap();
    let res = client.request(req).await.ok()?;
    if res.status() != 200 {
        return None;
    }
    let buf = hyper::body::to_bytes(res.into_body()).await.ok()?;
    String::from_utf8(buf.to_vec()).ok()
}
async fn get_config_tags(config: &Ini) -> HashMap<String, String>
{
    let default = config
        .get_map()
        .unwrap_or_default()
        .get("default")
        .unwrap()
        .clone();
    const TAG_POSTFIX : &str = "_tag";
    let tags = default.iter()
        .filter_map(|(k, v)| { 
            if !k.ends_with(TAG_POSTFIX) { return None; }
            let k_len = k.len() - TAG_POSTFIX.len();
            let k_short = String::from_str(&k[..k_len]).unwrap();
            return Some((k_short, v.clone().unwrap_or_default()));
        })
        .collect();
    return tags;
}
async fn get_repo_path(config: &Ini, project: &str) -> String {
    const POSTFIX_TO_REMOVE : &str = ".git";
    const PREFIX_TO_REMOVE : &str = "https://github.com/";
    let url_key = format!("{}_repo_url", project);
    let mut repo_path = config
        .get("default", url_key.as_str())
        .unwrap_or_default();
    if repo_path.ends_with(POSTFIX_TO_REMOVE) {
        repo_path = String::from(&repo_path[..repo_path.len()-POSTFIX_TO_REMOVE.len()]);
    }
    if repo_path.starts_with(PREFIX_TO_REMOVE) {
        repo_path = String::from(&repo_path[PREFIX_TO_REMOVE.len()..]);
    }
    return repo_path;
}
async fn update_dep(project: &str, oldtag: &str, config: &Ini, token: &str) -> Version {
    let repo_path = get_repo_path(config, project).await;
    let tags_json = get_repo_tags_json(&repo_path, token).await.unwrap();
    let tags = parse_tags_json(&tags_json).await.unwrap();
    let tags_str = tags.iter()
        .map(|s| {
            if s.starts_with("v") {
                return &s[1..];
            }
            return &s;
        })
        .collect();
    let fallback = Version::new(0, 0, 0);
    let latest = find_latest_version(tags_str).await.unwrap_or_else(|| fallback);
    return latest;
}
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut token = env::var("GITHUB_TOKEN").unwrap();
    token.pop();
    let config_content = fs::read_to_string("../config.mk").await.expect("could not read ../config.mk");
    let mut config = Ini::new();
    config.read(config_content).expect("couldnt parse config");
    let oldtags = get_config_tags(&config).await;
    for (project, oldtag) in oldtags.iter() {
        let latest = update_dep(&project, oldtag, &config, &token).await;
        let mut prefix = String::from("");
        if oldtag.starts_with("v") {
            prefix = String::from("v");
        }
        println!("{} should be at {}{} (was: {})", project, prefix, latest, oldtag);
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_find_latest_version() {
        let versions = vec!["1.2.3"];
        let latest = find_latest_version(versions).await.unwrap();
        assert_eq!(latest, Version::parse("1.2.3").unwrap());
    }
    #[tokio::test]
    async fn test_parse_tags_json() {
        let json = "
        [
        {
            \"commit\": {
              \"sha\": \"5d305789a86bc9a3d8a352522b219396ad4f3930\",
              \"url\": \"https://api.github.com/repos/bjoernmichaelsen/core/commits/5d305789a86bc9a3d8a352522b219396ad4f3930\"
            },
            \"name\": \"1.0.0\",
            \"node_id\": \"MDM6UmVmMzI4ODUxNTQ6cmVmcy90YWdzL3N1c2UtNC4wLTE=\",
            \"tarball_url\": \"https://api.github.com/repos/bjoernmichaelsen/core/tarball/refs/tags/suse-4.0-1\",
            \"zipball_url\": \"https://api.github.com/repos/bjoernmichaelsen/core/zipball/refs/tags/suse-4.0-1\"
        },
        {
            \"commit\": {
              \"sha\": \"5d305789a86bc9a3d8a352522b219396ad4f3931\",
              \"url\": \"https://api.github.com/repos/bjoernmichaelsen/core/commits/5d305789a86bc9a3d8a352522b219396ad4f3931\"
            },
            \"name\": \"1.2.3\",
            \"node_id\": \"MDM6UmVmMzI4ODUxNTQ6cmVmcy90YWdzL3N1c2UtNC4wLTF=\",
            \"tarball_url\": \"https://api.github.com/repos/bjoernmichaelsen/core/tarball/refs/tags/suse-4.0-2\",
            \"zipball_url\": \"https://api.github.com/repos/bjoernmichaelsen/core/zipball/refs/tags/suse-4.0-2\"
        }
        ]";
        let expected = "1.0.0, 1.2.3";
        let actual = parse_tags_json(json).await.expect("This should parse.");
        assert_eq!(join(actual.iter(), ", "), expected);
    }
    static CONFIG_CONTENT : &str = "
# this config should be kept parsable by POSIX sh, make and configparser (as ini file)
NERDFONTS_BASEURL=https://github.com/ryanoasis/nerd-fonts/releases/download/
NERDFONTS_VERSION=v3.0.2
NERDFONTS_NAMES=3270 Hack Monoid

NUSHELL_REPO_URL=https://github.com/nushell/nushell
NUSHELL_BUILD_DEPS=build-essential openssl pkg-config libssl-dev

STARSHIP_REPO_URL=https://github.com/starship/starship
STARSHIP_BUILD_DEPS=build-essential cmake

CARAPACE_REPO_URL=https://github.com/rsteube/carapace-bin.git
CARAPACE_BUILD_DEPS=

NEOVIM_ENABLED=T
NEOVIM_REPO_URL=https://github.com/neovim/neovim
NEOVIM_BUILD_DEPS=build-essential file ninja-build gettext cmake unzip curl

FNM_ENABLED=T # fnm is only installed if this is nonempty
FNM_REPO_URL=https://github.com/Schniz/fnm.git
FNM_BUILD_DEPS=build-essential

# rust is mostly statically linked, but you want a Debian or Ubuntu base image with the same glibc of your target machine.
RUST_BASE_IMAGE=debian:bookworm
# golang is mostly statically linked, but you want a Debian or Ubuntu base image with the same glibc of your target machine.
GOLANG_BASE_IMAGE=docker.io/golang:1.20.6-bookworm

C_BASE_IMAGE=debian:bookworm

# VERSIONS AUTOUPDATED BY DEPUP (manual edits will be overridden beyond this point)
NUSHELL_TAG=0.82.0
STARSHIP_TAG=v1.15.0
CARAPACE_TAG=v0.25.1
NEOVIM_TAG=v0.9.1
FNM_TAG=v1.35.0
    ";
    #[tokio::test]
    async fn test_parse_config_versions() {
        let mut config = Ini::new();
        config.read(String::from(CONFIG_CONTENT)).expect("should parse");
        let actual = get_config_tags(&config).await;
        assert_eq!(actual.get("nushell").unwrap(), "0.82.0");
        assert_eq!(actual.get("fnm").unwrap(), "v1.35.0");
        assert_eq!(actual.get("carapace").unwrap(), "v0.25.1");
        assert_eq!(actual.get("starship").unwrap(), "v1.15.0");
        assert_eq!(actual.get("neovim").unwrap(), "v0.9.1");
        assert_eq!(actual.len(), 5);
    }
    #[tokio::test]
    async fn test_parse_config_path() {
        let mut config = Ini::new();
        config.read(String::from(CONFIG_CONTENT)).expect("should parse");
        assert_eq!(
            get_repo_path(&config, "nushell").await.as_str(),
            "nushell/nushell");
        assert_eq!(
            get_repo_path(&config, "fnm").await.as_str(),
            "Schniz/fnm");
        assert_eq!(
            get_repo_path(&config, "carapace").await.as_str(),
            "rsteube/carapace-bin");
        assert_eq!(
            get_repo_path(&config, "starship").await.as_str(),
            "starship/starship");
        assert_eq!(
            get_repo_path(&config, "neovim").await.as_str(),
            "neovim/neovim");
    }
}