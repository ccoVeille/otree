use anyhow::{Context, Result};
use serde_json::{Map, Number, Value};
use toml::Value as TomlValue;

#[inline(always)]
pub(super) fn parse(data: &str) -> Result<Value> {
    let toml_value: TomlValue = toml::from_str(data).context("parse toml")?;
    Ok(convert_toml_value(toml_value))
}

#[inline(always)]
fn convert_toml_value(value: TomlValue) -> Value {
    match value {
        TomlValue::String(s) => Value::String(s),
        TomlValue::Integer(num) => Value::Number(Number::from(num)),
        TomlValue::Datetime(datetime) => Value::String(datetime.to_string()),
        TomlValue::Boolean(b) => Value::Bool(b),
        TomlValue::Float(num) => Value::Number(Number::from_f64(num).unwrap()),
        TomlValue::Array(values) => {
            let converted_values: Vec<Value> = values.into_iter().map(convert_toml_value).collect();
            Value::Array(converted_values)
        }
        TomlValue::Table(table) => {
            let mut object = Map::with_capacity(table.len());
            for (key, value) in table {
                object.insert(key, convert_toml_value(value));
            }
            Value::Object(object)
        }
    }
}

#[cfg(test)]
mod toml_test {
    use serde_json::Value;

    #[test]
    fn test_parse_toml() {
        const TOML_DATA: &str = r#"
            title = "TOML Example"
            conn = 123
            cap = 3.14
            retry = true
            habits = ["reading", "programming"]

            owner = {name = "Tom Preston-Werner", dob = 1979-05-27T07:32:00Z}

            [database]
            server = "127.0.0.1:1234"
            user = "root"
            password = "test password"

            [[persons]]
            name = "Alice"
            age = 20

            [[persons]]
            name = "Bob"
            age = 30

        "#;

        const JSON_DATA: &str = r#"
            {
                "title": "TOML Example",
                "conn": 123,
                "cap": 3.14,
                "retry": true,
                "habits": ["reading", "programming"],
                "owner": {
                    "name": "Tom Preston-Werner",
                    "dob": "1979-05-27T07:32:00Z"
                },
                "database": {
                    "server": "127.0.0.1:1234",
                    "user": "root",
                    "password": "test password"
                },
                "persons": [
                    {
                        "name": "Alice",
                        "age": 20
                    },
                    {
                        "name": "Bob",
                        "age": 30
                    }
                ]
            }
        "#;

        let from_toml = super::parse(TOML_DATA).unwrap();
        let from_json: Value = serde_json::from_str(JSON_DATA).unwrap();

        assert_eq!(from_toml, from_json);
    }
}
