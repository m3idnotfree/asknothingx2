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
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrEmpty<T> {
        Value(T),
        Empty(String),
    }
    match StringOrEmpty::deserialize(deserializer)? {
        StringOrEmpty::Value(value) => Ok(Some(value)),
        StringOrEmpty::Empty(s) if s.is_empty() => Ok(None),
        StringOrEmpty::Empty(s) => T::deserialize(s.into_deserializer()).map(Some),
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
