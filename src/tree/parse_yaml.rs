use anyhow::{Context, Result};
use serde_yaml::Value as YamlValue;

use super::{TreeValue, Value};

#[inline(always)]
pub(super) fn parse(data: &str) -> Result<Value> {
    let value = serde_yaml::from_str(data).context("parse yaml")?;
    Ok(Value::Yaml(value))
}

#[inline(always)]
pub(super) fn to_tree(value: YamlValue) -> TreeValue {
    match value {
        YamlValue::Null => TreeValue::Null,
        YamlValue::String(s) => TreeValue::String(s),
        YamlValue::Number(number) => TreeValue::Number(number.to_string()),
        YamlValue::Bool(b) => TreeValue::Bool(b),
        YamlValue::Sequence(arr) => {
            let yaml_str = serde_yaml::to_string(&arr).unwrap();
            TreeValue::Array(arr.into_iter().map(Value::Yaml).collect(), yaml_str)
        }
        YamlValue::Mapping(obj) => {
            let yaml_str = serde_yaml::to_string(&obj).unwrap();
            TreeValue::Object(
                obj.into_iter()
                    .map(|(key, value)| (object_field(key), Value::Yaml(value)))
                    .collect(),
                yaml_str,
            )
        }
        // TODO: We does not support yaml tag now.
        YamlValue::Tagged(_) => TreeValue::Null,
    }
}

fn object_field(value: YamlValue) -> String {
    match value {
        YamlValue::String(s) => s,
        YamlValue::Number(number) => number.to_string(),
        YamlValue::Bool(b) => b.to_string(),
        // We donot allow to use other type as object field name.
        _ => String::new(),
    }
}
