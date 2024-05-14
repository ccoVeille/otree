mod parse_json;
mod parse_toml;
mod parse_yaml;

use std::borrow::Cow;
use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::{bail, Result};
use ratatui::text::{Line, Span};
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use toml::Value as TomlValue;
use tui_tree_widget::TreeItem;

use crate::config::Config;

enum Value {
    Json(JsonValue),
    Yaml(YamlValue),
    Toml(TomlValue),
}

enum TreeValue {
    Null,
    String(String),
    Number(String),
    Bool(bool),
    Array(Vec<Value>, String),
    Object(Vec<(String, Value)>, String),
}

impl From<Value> for TreeValue {
    fn from(value: Value) -> Self {
        match value {
            Value::Json(value) => parse_json::to_tree(value),
            Value::Yaml(value) => parse_yaml::to_tree(value),
            Value::Toml(value) => parse_toml::to_tree(value),
        }
    }
}

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
            Self::Json => parse_json::parse(s),
            Self::Yaml => parse_yaml::parse(s),
            Self::Toml => parse_toml::parse(s),
        }
    }
}

pub struct Tree<'a> {
    pub items: Vec<TreeItem<'a, PathBuf>>,
    pub details: HashMap<PathBuf, String>,
}

impl<'a> Tree<'a> {
    pub fn parse(cfg: &'a Config, data: &str, content_type: Option<ContentType>) -> Result<Self> {
        let value: TreeValue = Self::parse_value(data, content_type)?.into();
        let mut details: HashMap<PathBuf, String> = HashMap::new();

        let items: Vec<TreeItem<PathBuf>> = if let TreeValue::Array(arr, _detail) = value {
            arr.into_iter()
                .enumerate()
                .map(|(idx, value)| {
                    let path = PathBuf::from(idx.to_string());
                    Self::convert_tree(cfg, path, value.into(), &mut details)
                })
                .collect()
        } else if let TreeValue::Object(obj, _detail) = value {
            obj.into_iter()
                .map(|(field, value)| {
                    let path = PathBuf::from(field);
                    Self::convert_tree(cfg, path, value.into(), &mut details)
                })
                .collect()
        } else {
            vec![Self::convert_tree(
                cfg,
                PathBuf::from("root"),
                value,
                &mut details,
            )]
        };

        Ok(Self { items, details })
    }

    fn parse_value(data: &str, content_type: Option<ContentType>) -> Result<Value> {
        if let Some(ct) = content_type {
            return ct.parse(data);
        }

        let auto_types = [ContentType::Json, ContentType::Yaml, ContentType::Toml];
        for auto_type in auto_types {
            if let Ok(value) = auto_type.parse(data) {
                return Ok(value);
            }
        }

        bail!("all parsers failed for input, unknown format");
    }

    fn convert_tree(
        cfg: &'a Config,
        path: PathBuf,
        value: TreeValue,
        details: &mut HashMap<PathBuf, String>,
    ) -> TreeItem<'a, PathBuf> {
        let (icon, value, detail, children) = match value {
            TreeValue::Null => (
                cfg.icons.null.as_str(),
                Cow::Borrowed("null"),
                String::new(),
                None,
            ),
            TreeValue::String(s) => (
                cfg.icons.str.as_str(),
                Cow::Owned(format!("{s:?}")),
                s,
                None,
            ),
            TreeValue::Number(number) => {
                let detail = number.clone();
                (cfg.icons.number.as_str(), Cow::Owned(number), detail, None)
            }
            TreeValue::Bool(b) => (
                cfg.icons.bool.as_str(),
                Cow::Owned(b.to_string()),
                b.to_string(),
                None,
            ),
            TreeValue::Array(arr, detail) => {
                let word = if arr.len() > 1 { "items" } else { "item" };
                let value = Cow::Owned(format!("{} {word}", arr.len()));
                let children: Vec<_> = arr
                    .into_iter()
                    .enumerate()
                    .map(|(idx, item)| {
                        let child_path = path.join(idx.to_string());
                        Self::convert_tree(cfg, child_path, item.into(), details)
                    })
                    .collect();
                (cfg.icons.array.as_str(), value, detail, Some(children))
            }
            TreeValue::Object(obj, detail) => {
                let word = if obj.len() > 1 { "fields" } else { "field" };
                let value = Cow::Owned(format!("{} {word}", obj.len()));
                let children: Vec<_> = obj
                    .into_iter()
                    .map(|(field, item)| {
                        let child_path = path.join(field);
                        Self::convert_tree(cfg, child_path, item.into(), details)
                    })
                    .collect();
                (cfg.icons.object.as_str(), value, detail, Some(children))
            }
        };

        details.insert(path.clone(), detail);

        let name = path
            .file_name()
            .map(|name| name.to_str().unwrap().to_string())
            .unwrap();

        let value = format!("= {value}");

        let line = Line::from(vec![
            Span::styled(name, cfg.tree_platte.name),
            Span::raw(" "),
            Span::styled(icon, cfg.tree_platte.icon),
            Span::raw(" "),
            Span::styled(value, cfg.tree_platte.value),
        ]);

        match children {
            Some(children) => TreeItem::new(path, line, children).unwrap(),
            None => TreeItem::new_leaf(path, line),
        }
    }
}
