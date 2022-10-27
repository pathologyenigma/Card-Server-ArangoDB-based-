use std::marker::PhantomData;

use serde::{de::{DeserializeOwned, Visitor, SeqAccess, self, MapAccess}, Deserialize};

pub mod gql;
#[derive(Debug)]
pub struct Response<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<graphql_client::Error>>,
}
impl<T> Response<T>
where
    T: DeserializeOwned,
{
    pub fn new(data: Option<T>, errors: Option<Vec<graphql_client::Error>>) -> Self {
        Self { data, errors }
    }
}
impl<'de, T> Deserialize<'de> for Response<T>
where
    T: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Data,
            Errors,
        }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;
                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("data or errors")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "data" => Ok(Field::Data),
                            "errors" => Ok(Field::Errors),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        struct ResponseVisitor<T> {
            phantom: PhantomData<T>,
        }
        impl<T> Default for ResponseVisitor<T> {
            fn default() -> Self {
                Self {
                    phantom: Default::default(),
                }
            }
        }
        impl<'de, T> Visitor<'de> for ResponseVisitor<T>
        where
            T: DeserializeOwned,
        {
            type Value = Response<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Response")
            }
            fn visit_seq<V>(self, mut seq: V) -> Result<Response<T>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let data = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let errors = Ok(seq.next_element()?.unwrap_or(None))?;

                Ok(Response::<T>::new(data, errors))
            }
            fn visit_map<V>(self, mut map: V) -> Result<Response<T>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut data = None;
                let mut errors = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Data => {
                            if data.is_some() {
                                return Err(de::Error::duplicate_field("data"));
                            }
                            data = Some(map.next_value()?);
                        }
                        Field::Errors => {
                            if errors.is_some() {
                                return Err(de::Error::duplicate_field("errors"));
                            }
                            errors = Some(map.next_value()?);
                        }
                    }
                }
                let data = data.ok_or_else(|| de::Error::missing_field("data"))?;
                let errors = Ok(errors.unwrap_or(None))?;
                Ok(Response::<T>::new(data, errors))
            }
        }
        const FIELDS: &[&str] = &["data", "errors"];
        deserializer.deserialize_struct("Response", FIELDS, ResponseVisitor::<T>::default())
    }
}