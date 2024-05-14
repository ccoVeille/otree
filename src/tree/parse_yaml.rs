use anyhow::{bail, Context, Result};
use serde::Deserialize;
use serde_yaml::Value as YamlValue;

use super::{TreeValue, Value};

#[inline(always)]
pub(super) fn parse(data: &str) -> Result<Value> {
    let mut values = Vec::with_capacity(1);
    for document in serde_yaml::Deserializer::from_str(data) {
        let value = YamlValue::deserialize(document).context("parse yaml")?;
        values.push(value);
    }
    if values.is_empty() {
        bail!("no document in yaml data");
    }
    if values.len() == 1 {
        return Ok(Value::Yaml(values.into_iter().next().unwrap()));
    }

    Ok(Value::Yaml(YamlValue::Sequence(values)))
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
