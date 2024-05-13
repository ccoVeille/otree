mod toml;
mod yaml;

use anyhow::{bail, Context, Result};
use serde_json::Value;

#[derive(Debug, Clone, Copy)]
pub enum ContentType {
    Json,
    Yaml,
    Toml,
}

impl ContentType {
    #[inline(always)]
    fn parse(&self, s: &str) -> Result<Value> {
        match self {
            Self::Json => serde_json::from_str(s).context("parse json"),
            Self::Toml => crate::parse::toml::parse(s),
            Self::Yaml => yaml::parse(s),
        }
    }
}

pub fn parse_str(s: &str, content_type: Option<ContentType>) -> Result<Value> {
    if let Some(ct) = content_type {
        return ct.parse(s);
    }

    let auto_types = [ContentType::Json, ContentType::Yaml, ContentType::Toml];
    for auto_type in auto_types {
        if let Ok(value) = auto_type.parse(s) {
            return Ok(value);
        }
    }

    bail!("all parsers failed for input, unknown format");
}
