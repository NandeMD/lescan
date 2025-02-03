use base64::{alphabet, engine, Engine as _};
use serde::{de::Visitor, Deserializer, Serializer};

const B64: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, engine::general_purpose::NO_PAD);

pub mod b64_img_overwrite {
    use super::*;

    pub fn serialize<S>(img: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let encoded = B64.encode(img);
        serializer.serialize_str(&encoded)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize_to_vecu8(deserializer)
    }
}

pub fn deserialize_to_vecu8<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    struct Base64Visitor;

    impl Visitor<'_> for Base64Visitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a base64 encoded string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            B64.decode(v).map_err(serde::de::Error::custom)
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            B64.decode(&v).map_err(serde::de::Error::custom)
        }
    }

    deserializer.deserialize_any(Base64Visitor)
}
