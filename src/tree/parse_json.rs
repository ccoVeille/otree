use anyhow::{Context, Result};
use serde_json::Value as JsonValue;

use super::{TreeValue, Value};

#[inline(always)]
pub(super) fn parse(data: &str) -> Result<Value> {
    let value = serde_json::from_str(data).context("parse json")?;
    Ok(Value::Json(value))
}

#[inline(always)]
pub(super) fn to_tree(value: JsonValue) -> TreeValue {
    match value {
        JsonValue::Null => TreeValue::Null,
        JsonValue::String(s) => TreeValue::String(s),
        JsonValue::Number(number) => TreeValue::Number(number.to_string()),
        JsonValue::Bool(b) => TreeValue::Bool(b),
        JsonValue::Array(arr) => {
            let json_str = serde_json::to_string(&arr).unwrap();
            TreeValue::Array(arr.into_iter().map(Value::Json).collect(), json_str)
        }
        JsonValue::Object(obj) => {
            let json_str = serde_json::to_string(&obj).unwrap();
            TreeValue::Object(
                obj.into_iter()
                    .map(|(key, value)| (key, Value::Json(value)))
                    .collect(),
                json_str,
            )
        }
    }
}
