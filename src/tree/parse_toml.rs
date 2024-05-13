use anyhow::{Context, Result};
use toml::Value as TomlValue;

use super::{TreeValue, Value};

#[inline(always)]
pub(super) fn parse(data: &str) -> Result<Value> {
    let value = toml::from_str(data).context("parse toml")?;
    Ok(Value::Toml(value))
}

#[inline(always)]
pub(super) fn to_tree(value: TomlValue) -> TreeValue {
    match value {
        TomlValue::String(s) => TreeValue::String(s),
        TomlValue::Datetime(datetime) => TreeValue::String(datetime.to_string()),
        TomlValue::Integer(i) => TreeValue::Number(i.to_string()),
        TomlValue::Float(f) => TreeValue::Number(f.to_string()),
        TomlValue::Boolean(b) => TreeValue::Bool(b),
        TomlValue::Array(arr) => {
            let toml_str = toml::to_string_pretty(&arr).unwrap();
            TreeValue::Array(arr.into_iter().map(Value::Toml).collect(), toml_str)
        }
        TomlValue::Table(table) => {
            let toml_str = toml::to_string_pretty(&table).unwrap();
            TreeValue::Object(
                table
                    .into_iter()
                    .map(|(key, value)| (key, Value::Toml(value)))
                    .collect(),
                toml_str,
            )
        }
    }
}
