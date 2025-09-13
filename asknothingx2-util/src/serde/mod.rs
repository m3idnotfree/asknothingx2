use serde::{de::IntoDeserializer, Deserialize, Deserializer, Serialize, Serializer};

/// https://github.com/serde-rs/serde/issues/2362
pub fn deserialize_empty_object_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    #[derive(Deserialize)]
    #[serde(
        untagged,
        deny_unknown_fields,
        expecting = "object, empty object or null"
    )]
    enum Helper<T> {
        Data(T),
        Empty {},
        Null,
    }
    match Helper::deserialize(deserializer) {
        Ok(Helper::Data(data)) => Ok(Some(data)),
        Ok(_) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn deserialize_empty_string_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) if s.is_empty() => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some),
        None => Ok(None),
    }
}

pub fn serialize_none_as_empty_string<T, S>(
    value: &Option<T>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    match value {
        None => serializer.serialize_str(""),
        Some(v) => v.serialize(serializer),
    }
}

pub fn serialize_none_as_empty_object<S, T>(
    value: &Option<T>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    match value {
        Some(v) => v.serialize(serializer),
        None => {
            use serde::ser::SerializeMap;
            let map = serializer.serialize_map(Some(0))?;
            map.end()
        }
    }
}

pub fn deserialize_empty_array_as_none<'de, D, T>(
    deserializer: D,
) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let vec: Vec<T> = Vec::deserialize(deserializer)?;
    Ok(if vec.is_empty() { None } else { Some(vec) })
}

pub fn serialize_none_as_empty_array<S, T>(
    value: &Option<Vec<T>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    match value {
        Some(v) => v.serialize(serializer),
        None => {
            use serde::ser::SerializeSeq;
            let seq = serializer.serialize_seq(Some(0))?;
            seq.end()
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::serde::{
        deserialize_empty_array_as_none, deserialize_empty_object_as_none,
        deserialize_empty_string_as_none, serialize_none_as_empty_array,
        serialize_none_as_empty_object, serialize_none_as_empty_string,
    };

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct Struct {
        name: String,
        value: i32,
    }

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct EmptyObject {
        #[serde(deserialize_with = "deserialize_empty_object_as_none")]
        #[serde(serialize_with = "serialize_none_as_empty_object")]
        data: Option<Struct>,
    }

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct EmptyString {
        #[serde(deserialize_with = "deserialize_empty_string_as_none")]
        #[serde(serialize_with = "serialize_none_as_empty_string")]
        value: Option<String>,
    }

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct EmptyArray {
        #[serde(deserialize_with = "deserialize_empty_array_as_none")]
        #[serde(serialize_with = "serialize_none_as_empty_array")]
        items: Option<Vec<String>>,
    }

    mod empty_object {
        use crate::serde::tests::{EmptyObject, Struct};

        #[test]
        fn deserialize_empty_object_as_none() {
            let json = r#"{"data": {}}"#;
            let result: EmptyObject = serde_json::from_str(json).unwrap();
            assert_eq!(result.data, None);
        }

        #[test]
        fn deserialize_null_as_none() {
            let json = r#"{"data": null}"#;
            let result: EmptyObject = serde_json::from_str(json).unwrap();
            assert_eq!(result.data, None);
        }

        #[test]
        fn deserialize_valid_object() {
            let json = r#"{"data": {"name": "test", "value": 62}}"#;
            let result: EmptyObject = serde_json::from_str(json).unwrap();
            assert_eq!(
                result.data,
                Some(Struct {
                    name: "test".to_string(),
                    value: 62
                })
            );
        }

        #[test]
        fn deserialize_invalid_object_fails() {
            let json = r#"{"data": {"unknown_field": "value"}}"#;
            let result: Result<EmptyObject, _> = serde_json::from_str(json);
            assert!(result.is_err());
        }

        #[test]
        fn serialize_none_as_empty_object() {
            let test_data = EmptyObject { data: None };
            let json = serde_json::to_string(&test_data).unwrap();
            assert_eq!(json, r#"{"data":{}}"#);
        }

        #[test]
        fn serialize_some_as_object() {
            let test_data = EmptyObject {
                data: Some(Struct {
                    name: "test".to_string(),
                    value: 62,
                }),
            };
            let json = serde_json::to_string(&test_data).unwrap();
            assert_eq!(json, r#"{"data":{"name":"test","value":62}}"#);
        }

        #[test]
        fn roundtrip_none() {
            let original = EmptyObject { data: None };
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: EmptyObject = serde_json::from_str(&json).unwrap();
            assert_eq!(original, deserialized);
        }

        #[test]
        fn roundtrip_some() {
            let original = EmptyObject {
                data: Some(Struct {
                    name: "test".to_string(),
                    value: 62,
                }),
            };
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: EmptyObject = serde_json::from_str(&json).unwrap();
            assert_eq!(original, deserialized);
        }
    }

    mod empty_string {
        use crate::serde::tests::EmptyString;

        #[test]
        fn deserialize_empty_string_as_none() {
            let json = r#"{"value": ""}"#;
            let result: EmptyString = serde_json::from_str(json).unwrap();
            assert_eq!(result.value, None);
        }

        #[test]
        fn deserialize_null_string_as_none() {
            let json = r#"{"value": null}"#;
            let result: EmptyString = serde_json::from_str(json).unwrap();
            assert_eq!(result.value, None);
        }

        #[test]
        fn deserialize_valid_string() {
            let json = r#"{"value": "hello"}"#;
            let result: EmptyString = serde_json::from_str(json).unwrap();
            assert_eq!(result.value, Some("hello".to_string()));
        }

        #[test]
        fn deserialize_whitespace_string() {
            let json = r#"{"value": " "}"#;
            let result: EmptyString = serde_json::from_str(json).unwrap();
            assert_eq!(result.value, Some(" ".to_string()));
        }

        #[test]
        fn serialize_none_as_empty_string() {
            let test_data = EmptyString { value: None };
            let json = serde_json::to_string(&test_data).unwrap();
            assert_eq!(json, r#"{"value":""}"#);
        }

        #[test]
        fn serialize_some_as_string() {
            let test_data = EmptyString {
                value: Some("hello".to_string()),
            };
            let json = serde_json::to_string(&test_data).unwrap();
            assert_eq!(json, r#"{"value":"hello"}"#);
        }

        #[test]
        fn roundtrip_none() {
            let original = EmptyString { value: None };
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: EmptyString = serde_json::from_str(&json).unwrap();
            assert_eq!(original, deserialized);
        }

        #[test]
        fn roundtrip_some() {
            let original = EmptyString {
                value: Some("test".to_string()),
            };
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: EmptyString = serde_json::from_str(&json).unwrap();
            assert_eq!(original, deserialized);
        }
    }

    mod empty_array {
        use crate::serde::tests::EmptyArray;

        #[test]
        fn deserialize_empty_array_as_none() {
            let json = r#"{"items": []}"#;
            let result: EmptyArray = serde_json::from_str(json).unwrap();
            assert_eq!(result.items, None);
        }

        #[test]
        fn deserialize_valid_array() {
            let json = r#"{"items": ["a", "b", "c"]}"#;
            let result: EmptyArray = serde_json::from_str(json).unwrap();
            assert_eq!(
                result.items,
                Some(vec!["a".to_string(), "b".to_string(), "c".to_string()])
            );
        }

        #[test]
        fn deserialize_single_item_array() {
            let json = r#"{"items": ["single"]}"#;
            let result: EmptyArray = serde_json::from_str(json).unwrap();
            assert_eq!(result.items, Some(vec!["single".to_string()]));
        }

        #[test]
        fn serialize_none_as_empty_array() {
            let test_data = EmptyArray { items: None };
            let json = serde_json::to_string(&test_data).unwrap();
            assert_eq!(json, r#"{"items":[]}"#);
        }

        #[test]
        fn serialize_some_as_array() {
            let test_data = EmptyArray {
                items: Some(vec!["a".to_string(), "b".to_string()]),
            };
            let json = serde_json::to_string(&test_data).unwrap();
            assert_eq!(json, r#"{"items":["a","b"]}"#);
        }

        #[test]
        fn serialize_empty_vec_as_array() {
            let test_data = EmptyArray {
                items: Some(Vec::new()),
            };
            let json = serde_json::to_string(&test_data).unwrap();
            assert_eq!(json, r#"{"items":[]}"#);
        }

        #[test]
        fn roundtrip_none() {
            let original = EmptyArray { items: None };
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: EmptyArray = serde_json::from_str(&json).unwrap();
            assert_eq!(original, deserialized);
        }

        #[test]
        fn roundtrip_some() {
            let original = EmptyArray {
                items: Some(vec!["test".to_string(), "data".to_string()]),
            };
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: EmptyArray = serde_json::from_str(&json).unwrap();
            assert_eq!(original, deserialized);
        }
    }
}
