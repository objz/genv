use std::collections::BTreeMap;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use color_eyre::eyre::{eyre, Result};

use crate::logging as log;

pub type EnvMap = BTreeMap<String, String>;

pub fn default_env_file(custom: Option<&Path>) -> Result<PathBuf> {
    if let Some(c) = custom {
        return Ok(c.to_path_buf());
    }
    let mut p = home::home_dir().ok_or_else(|| eyre!("cannot resolve home directory"))?;
    p.push(".config/envm");
    Ok(p.join("env"))
}

pub fn load_env_map(path: &Path) -> Result<EnvMap> {
    let mut map = EnvMap::new();
    if !path.exists() {
        return Ok(map);
    }
    let f = fs::File::open(path)?;
    let reader = BufReader::new(f);

    for (lineno, line) in reader.lines().enumerate() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = split_once_eq(trimmed) {
            let key = k.trim().to_string();
            let val = v.trim().to_string();
            if let Err(e) = validate_key(&key) {
                log::warn(format!("line {}: {}", lineno + 1, e));
                continue;
            }
            map.insert(key, val);
        } else {
            log::warn(format!(
                "skipping malformed line in {}: {}",
                path.display(),
                line
            ));
        }
    }
    Ok(map)
}

pub fn save_env_map(path: &Path, map: &EnvMap) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut f = fs::File::create(path)?;
    writeln!(f, "# Managed by envm")?;
    for (k, v) in map {
        writeln!(f, "{k}={v}")?;
    }
    Ok(())
}

pub fn validate_key(key: &str) -> Result<()> {
    if key.is_empty() {
        return Err(eyre!("variable name cannot be empty"));
    }
    let mut chars = key.chars();
    let first_ok = chars
        .next()
        .map(|c| c == '_' || c.is_ascii_alphabetic())
        .unwrap_or(false);
    let rest_ok = chars.all(|c| c == '_' || c.is_ascii_alphanumeric());
    if !(first_ok && rest_ok) {
        return Err(eyre!(format!("invalid variable name: {}", key)));
    }
    Ok(())
}

fn split_once_eq(s: &str) -> Option<(&str, &str)> {
    let idx = s.find('=')?;
    Some((&s[..idx], &s[idx + 1..]))
}

pub fn quote_posix(val: &str) -> String {
    if val.is_empty() {
        return "''".to_string();
    }
    if !val.contains(['\'', '\n', '\r', '\t', ' '])
        && val
            .chars()
            .all(|c| !matches!(c, '$' | '"' | '`' | '!' | '\\'))
    {
        return val.to_string();
    }
    let mut out = String::from("'");
    for ch in val.chars() {
        if ch == '\'' {
            out.push_str("'\"'\"'");
        } else {
            out.push(ch);
        }
    }
    out.push('\'');
    out
}

pub fn quote_fish(val: &str) -> String {
    let escaped = val.replace('\'', "\\'");
    format!("'{}'", escaped)
}
