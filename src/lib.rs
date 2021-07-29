use serde::de::Deserialize;
use serde::de::Visitor;
use std::marker::PhantomData;

struct SeqVisitor<T>(PhantomData<T>);
pub struct Seq<T>(PhantomData<T>);

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Seq<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(SeqVisitor(PhantomData))
    }
}

impl<'de, T: Deserialize<'de>> Visitor<'de> for SeqVisitor<T> {
    type Value = Seq<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a Seq")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        loop {
            match seq.next_element::<T>() {
                Ok(None) => break,
                Err(e) => return Err(e),
                _ => (),
            }
        }
        Ok(Seq(PhantomData))
    }
}

struct MapVisitor<K, V>(PhantomData<(K, V)>);
pub struct Map<K, V>(PhantomData<(K, V)>);

impl<'de, K, V> Deserialize<'de> for Map<K, V>
where
    K: Deserialize<'de>,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(MapVisitor(PhantomData))
    }
}

impl<'de, K, V> Visitor<'de> for MapVisitor<K, V>
where
    K: Deserialize<'de>,
    V: Deserialize<'de>,
{
    type Value = Map<K, V>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a Map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        loop {
            match map.next_entry::<K, V>() {
                Ok(None) => break,
                Err(e) => return Err(e),
                _ => (),
            }
        }
        Ok(Map(PhantomData))
    }
}

struct StrVisitor;
pub struct Str;

impl<'de> Deserialize<'de> for Str {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(StrVisitor)
    }
}

impl<'de> Visitor<'de> for StrVisitor {
    type Value = Str;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a String")
    }

    fn visit_str<E>(self, _: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Str)
    }

    fn visit_string<E>(self, _: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Str)
    }
}

pub struct Any;
struct AnyVisitor;

impl<'de> Deserialize<'de> for Any {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(AnyVisitor)
    }
}

impl<'de> Visitor<'de> for AnyVisitor {
    type Value = Any;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a String")
    }

    fn visit_bool<E>(self, _v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Any)
    }

    fn visit_i8<E>(self, _v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Any)
    }

    fn visit_i16<E>(self, _v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Any)
    }

    fn visit_i32<E>(self, _v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Any)
    }

    fn visit_i64<E>(self, _v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Any)
    }

    fn visit_i128<E>(self, _v: i128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Any)
    }

    fn visit_u8<E>(self, _v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Any)
    }

    fn visit_u16<E>(self, _v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Any)
    }

    fn visit_u32<E>(self, _v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Any)
    }

    fn visit_u64<E>(self, _v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Any)
    }

    fn visit_u128<E>(self, _v: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Any)
    }

    fn visit_f32<E>(self, _v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Any)
    }

    fn visit_f64<E>(self, _v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Any)
    }

    fn visit_char<E>(self, _v: char) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Any)
    }

    fn visit_str<E>(self, _v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Any)
    }

    fn visit_borrowed_str<E>(self, _v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Any)
    }

    fn visit_string<E>(self, _v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Any)
    }

    fn visit_bytes<E>(self, _v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Any)
    }

    fn visit_borrowed_bytes<E>(self, _v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Any)
    }

    fn visit_byte_buf<E>(self, _v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Any)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Any)
    }

    fn visit_some<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Any)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Any)
    }

    fn visit_newtype_struct<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Any)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        loop {
            match seq.next_element::<Any>() {
                Ok(None) => break,
                Err(e) => return Err(e),
                _ => (),
            }
        }
        Ok(Any)
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        loop {
            match map.next_entry::<Any, Any>() {
                Ok(None) => break,
                Err(e) => return Err(e),
                _ => (),
            }
        }
        Ok(Any)
    }

    fn visit_enum<A>(self, _data: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        Ok(Any)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserialize_seq() {
        let json = json!([1]);
        let json = json.to_string();

        let _: Seq<i32> = serde_json::from_str(&json).unwrap();

        let json = json!(1);
        let json = json.to_string();

        assert!(serde_json::from_str::<Seq<i32>>(&json).is_err());
    }

    #[test]
    fn deserialize_str() {
        let json = json!("hello");
        let json = json.to_string();

        let _: Str = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn deserialize_map() {
        let json = json!({"hello": 1});
        let json = json.to_string();

        let _: Map<Str, i32> = serde_json::from_str(&json).unwrap();

        let json = json!(1);
        let json = json.to_string();

        assert!(serde_json::from_str::<Seq<i32>>(&json).is_err());
    }

    #[test]
    fn deserialize_any() {
        let json = json!({"hello": 1, "blabla": "hello"});
        let json = json.to_string();

        let _: Any = serde_json::from_str(&json).unwrap();

        // invalid json
        let json = r##"{"hello": {1: "blabla"}}"##;
        assert!(serde_json::from_str::<Seq<i32>>(&json).is_err());
    }
}
