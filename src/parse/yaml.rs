use anyhow::{Context, Result};
use serde_json::{Map, Number, Value};
use serde_yaml::Value as YamlValue;

#[inline(always)]
pub(super) fn parse(data: &str) -> Result<Value> {
    let yaml_value: YamlValue = serde_yaml::from_str(data).context("parse yaml")?;
    Ok(convert_yaml_value(yaml_value))
}

#[inline(always)]
fn convert_yaml_value(value: YamlValue) -> Value {
    match value {
        YamlValue::Null => Value::Null,
        YamlValue::Bool(b) => Value::Bool(b),
        YamlValue::String(s) => Value::String(s),
        YamlValue::Number(num) => {
            if let Some(num) = num.as_u64() {
                Value::Number(Number::from(num))
            } else if let Some(num) = num.as_i64() {
                Value::Number(Number::from(num))
            } else {
                Value::Number(Number::from_f64(num.as_f64().unwrap()).unwrap())
            }
        }
        YamlValue::Sequence(values) => {
            let converted_values: Vec<Value> = values.into_iter().map(convert_yaml_value).collect();
            Value::Array(converted_values)
        }
        YamlValue::Mapping(mapping) => {
            let mut object = Map::with_capacity(mapping.len());
            for (key, value) in mapping {
                object.insert(key.as_str().unwrap().to_string(), convert_yaml_value(value));
            }
            Value::Object(object)
        }
        // Ignore yaml tags
        YamlValue::Tagged(_) => Value::Null,
    }
}

#[cfg(test)]
mod yaml_test {
    use serde_json::Value;

    #[test]
    fn test_parse_yaml() {
        const YAML_DATA: &str = r#"
            title: "YAML Example"
            conn: 123
            cap: 3.14
            retry: true
            habits: ["reading", "programming"]
            owner: {name: "Tom Preston-Werner", dob: "1979-05-27T07:32:00Z"}
            database:
                server: "127.0.0.1:1234"
                user: "root"
                password: "test password"
            persons:
                - name: Alice
                  age: 20
                - name: Bob
                  age: 30
        "#;

        const JSON_DATA: &str = r#"
            {
                "title": "YAML Example",
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

        let from_yaml = super::parse(YAML_DATA).unwrap();
        let from_json: Value = serde_json::from_str(JSON_DATA).unwrap();

        assert_eq!(from_yaml, from_json);
    }
}
