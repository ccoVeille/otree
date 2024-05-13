use std::borrow::Cow;
use std::path::PathBuf;

use ratatui::text::{Line, Span};
use serde_json::Value;
use tui_tree_widget::{TreeItem, TreeState};

use crate::config::Config;

pub(super) struct Tree<'a> {
    state: TreeState<PathBuf>,
    items: Vec<TreeItem<'a, PathBuf>>,
}

impl<'a> Tree<'a> {
    pub(super) fn new(cfg: &'a Config, value: Value) -> Self {
        todo!()
    }
}

fn build_tree_item(cfg: &Config, path: PathBuf, value: Value) -> TreeItem<PathBuf> {
    let (icon, value, children) = match value {
        Value::Null => (cfg.icons.null.as_str(), Cow::Borrowed("null"), None),
        Value::String(s) => (cfg.icons.str.as_str(), Cow::Owned(format!("{s:?}")), None),
        Value::Number(num) => (cfg.icons.number.as_str(), Cow::Owned(num.to_string()), None),
        Value::Bool(b) => (cfg.icons.bool.as_str(), Cow::Owned(b.to_string()), None),
        Value::Array(arr) => {
            let word = if arr.len() > 1 { "items" } else { "items" };
            let value = Cow::Owned(format!("{} {word}", arr.len()));
            let children: Vec<_> = arr
                .into_iter()
                .enumerate()
                .map(|(idx, item)| {
                    let child_path = path.join(idx.to_string());
                    build_tree_item(cfg, child_path, item)
                })
                .collect();
            (cfg.icons.array.as_str(), value, Some(children))
        }
        Value::Object(object) => {
            let word = if object.len() > 1 { "fields" } else { "field" };
            let value = Cow::Owned(format!("{} {word}", object.len()));
            let children: Vec<_> = object
                .into_iter()
                .map(|(field, item)| {
                    let child_path = path.join(field);
                    build_tree_item(cfg, child_path, item)
                })
                .collect();

            (cfg.icons.object.as_str(), value, Some(children))
        }
    };

    let name = path
        .file_name()
        .map(|name| name.to_str().unwrap().to_string())
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed("root"));

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
