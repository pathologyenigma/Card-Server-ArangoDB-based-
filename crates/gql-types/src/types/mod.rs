pub mod card;
pub mod collection;
pub mod user;
pub mod common {
    use std::str::FromStr;
    pub fn jwt_encode(
        secret: &'static str,
        token_data: &TokenData,
    ) -> jsonwebtoken::errors::Result<String> {
        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &token_data,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
        )
    }
    pub fn jwt_decode(token: String, secret: &'static str) -> Result<TokenData, String> {
        let res = jsonwebtoken::decode::<TokenData>(
            &token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
            &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
        );
        res.map(|token| token.claims).map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken => "InvalidToken".to_string(),
            jsonwebtoken::errors::ErrorKind::InvalidSignature => "InvalidSignature".to_string(),
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => "ExpiredSignature".to_string(),
            jsonwebtoken::errors::ErrorKind::InvalidAudience => "InvalidAudience".to_string(),
            jsonwebtoken::errors::ErrorKind::InvalidSubject => "InvalidSubject".to_string(),
            jsonwebtoken::errors::ErrorKind::InvalidIssuer => "InvalidIssuer".to_string(),
            jsonwebtoken::errors::ErrorKind::ImmatureSignature => "ImmatureSignature".to_string(),
            jsonwebtoken::errors::ErrorKind::InvalidEcdsaKey => "InvalidEcdsaKey".to_string(),
            jsonwebtoken::errors::ErrorKind::InvalidRsaKey(msg) => {
                format!("InvalidRsaKey {}", msg).to_string()
            }
            jsonwebtoken::errors::ErrorKind::InvalidAlgorithmName => {
                "InvalidAlgorithmName".to_string()
            }
            jsonwebtoken::errors::ErrorKind::InvalidKeyFormat => "InvalidKeyFormat".to_string(),
            jsonwebtoken::errors::ErrorKind::InvalidAlgorithm => "InvalidAlgorithm".to_string(),
            jsonwebtoken::errors::ErrorKind::Base64(_)
            | jsonwebtoken::errors::ErrorKind::Json(_)
            | jsonwebtoken::errors::ErrorKind::Utf8(_)
            | jsonwebtoken::errors::ErrorKind::Crypto(_) => {
                "failed to parse token to json, check your token".to_string()
            }
            jsonwebtoken::errors::ErrorKind::RsaFailedSigning => "RsaFailedSigning".to_string(),
            jsonwebtoken::errors::ErrorKind::MissingRequiredClaim(msg) => {
                format!("MissingRequiredClaim: {}", msg)
            }
            jsonwebtoken::errors::ErrorKind::MissingAlgorithm => "MissingAlgorithm".to_string(),
            _ => "unknown error".to_string(),
        })
    }
    use async_graphql::{
        Enum, InputObject, InputValueError, InputValueResult, OneofObject, Scalar, ScalarType,
        SimpleObject, Value,
    };
    use chrono::{prelude::*, LocalResult};
    use serde::{de::Visitor, Deserialize, Serialize};
    use tracing::error;

    pub struct Longitude(pub f64);

    #[Scalar]
    /// A type representing Longitude
    impl ScalarType for Longitude {
        fn parse(value: Value) -> InputValueResult<Self> {
            if let Value::Number(value) = value {
                if value.is_f64() {
                    let value = value.as_f64().unwrap();
                    if value < 180. && value > -180. {
                        return Ok(Self(value));
                    }
                }
            }
            Err(InputValueError::custom("not a valid Longitude"))
        }

        fn to_value(&self) -> Value {
            Value::from(self.0)
        }
    }

    pub struct Latitude(pub f64);
    #[Scalar]
    /// a type representing a latitude
    impl ScalarType for Latitude {
        fn parse(value: Value) -> InputValueResult<Self> {
            if let Value::Number(value) = value {
                if value.is_f64() {
                    let value = value.as_f64().unwrap();
                    if value < 180. && value > -180. {
                        return Ok(Self(value));
                    }
                }
            }
            Err(InputValueError::custom("not a valid Latitude"))
        }

        fn to_value(&self) -> Value {
            Value::from(self.0)
        }
    }

    pub struct YearMonthBundle(pub i32, pub u8);
    impl FromStr for YearMonthBundle {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let slices = s.split("-").collect::<Vec<&str>>();
            if slices.len() != 2 {
                return Err("should be like ${year}-${month}".to_owned());
            }
            let (year, month) = (slices[0], slices[1]);
            let year = year.parse::<i32>().unwrap_or(0);
            let month = month.parse::<u8>().unwrap_or(0);
            if let LocalResult::Single(date) = Utc.ymd_opt(year, month as u32, 0) {
                if date.year() == 0 || date.month() == 0 {
                    return Err("invalid date".to_owned());
                }
                return Ok(Self(year, month));
            } else {
                return Err("not a valid date".to_owned());
            }
        }
    }
    #[Scalar]
    /// a type for year and month date values
    /// format like Y-m
    impl ScalarType for YearMonthBundle {
        fn parse(value: Value) -> InputValueResult<Self> {
            if let Value::String(value) = value {
                return Self::from_str(&value).map_err(|err| InputValueError::custom(err));
            }
            return Err(InputValueError::custom("not a valid date"));
        }

        fn to_value(&self) -> Value {
            Value::String(format!("{}-{}", self.0, self.1))
        }
    }
    impl Serialize for YearMonthBundle {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(&format!("{}-{}", self.0, self.1))
        }
    }
    impl<'de> Deserialize<'de> for YearMonthBundle {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct YMBVisitor;
            impl<'de> Visitor<'de> for YMBVisitor {
                type Value = YearMonthBundle;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("a str like this: year-month")
                }
                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Self::Value::from_str(v).expect("failed to parse from string"))
                }
            }
            deserializer.deserialize_str(YMBVisitor)
        }
    }

    pub struct YearMonthDayBundle(pub i32, pub u8, pub u8);
    impl FromStr for YearMonthDayBundle {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let slices = s.split("-").collect::<Vec<&str>>();
            if slices.len() != 3 {
                return Err("should be like ${year}-${month}".to_owned());
            }
            let (year, month, day) = (slices[0], slices[1], slices[2]);
            let year = year.parse::<i32>().unwrap_or(0);
            let month = month.parse::<u8>().unwrap_or(0);
            let day = day.parse::<u8>().unwrap_or(0);
            if let LocalResult::Single(date) = Utc.ymd_opt(year, month as u32, day as u32) {
                if date.year() == 0 || date.month() == 0 || date.day() == 0 {
                    return Err("invalid date".to_owned());
                }
                return Ok(Self(year, month, day));
            } else {
                return Err("not a valid date".to_owned());
            }
        }
    }
    #[Scalar]
    /// a type for year month and day date values
    /// format like Y-m-d
    impl ScalarType for YearMonthDayBundle {
        fn parse(value: Value) -> InputValueResult<Self> {
            if let Value::String(value) = value {
                return Self::from_str(value.as_str()).map_err(|err| InputValueError::custom(err));
            }
            return Err(InputValueError::custom("not a valid date"));
        }

        fn to_value(&self) -> Value {
            Value::String(format!("{}-{}-{}", self.0, self.1, self.2))
        }
    }
    impl Serialize for YearMonthDayBundle {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(&format!("{}-{}-{}", self.0, self.1, self.2))
        }
    }
    impl<'de> Deserialize<'de> for YearMonthDayBundle {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct YMDBVisitor;
            impl<'de> Visitor<'de> for YMDBVisitor {
                type Value = YearMonthDayBundle;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("a str like this: year-month-day")
                }
                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Self::Value::from_str(v).expect("failed to parse from string"))
                }
            }
            deserializer.deserialize_str(YMDBVisitor)
        }
    }

    pub struct DateTimeForGQL(pub DateTime<Utc>);
    impl FromStr for DateTimeForGQL {
        type Err = chrono::ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self(DateTime::<Utc>::from_str(s)?))
        }
    }
    impl Serialize for DateTimeForGQL {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(format!("{:?}", self.0).as_str())
        }
    }
    impl<'de> Deserialize<'de> for DateTimeForGQL {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct DateTimeForGQLVisitor;
            impl<'de> Visitor<'de> for DateTimeForGQLVisitor {
                type Value = DateTimeForGQL;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("expect iso string")
                }
                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Self::Value::from_str(v).expect("failed to parse from string"))
                }
            }
            deserializer.deserialize_str(DateTimeForGQLVisitor)
        }
    }
    impl DateTimeForGQL {
        pub fn now() -> Self {
            return Self(Utc::now());
        }
    }
    #[Scalar(name = "DateTime")]
    /// a full date time type
    /// it is a UTC format one
    impl ScalarType for DateTimeForGQL {
        fn parse(value: Value) -> InputValueResult<Self> {
            if let Value::String(value) = value {
                if let Ok(time) = DateTime::<Utc>::from_str(&value) {
                    return Ok(Self(time));
                }
            }
            return Err(InputValueError::custom("not a valid date time"));
        }

        fn to_value(&self) -> Value {
            Value::String(format!("{:?}", self.0))
        }
    }
    #[derive(Debug)]
    pub struct ID(pub u64);
    impl FromStr for ID {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self(u64::from_str(s).unwrap()))
        }
    }
    #[Scalar]
    /// id generated from arangodb
    impl ScalarType for ID {
        fn parse(value: Value) -> InputValueResult<Self> {
            if let Value::String(value) = value {
                if let Ok(uuid) = u64::from_str(&value) {
                    return Ok(Self(uuid));
                }
            }
            return Err(InputValueError::custom("not a valid uuid"));
        }

        fn to_value(&self) -> Value {
            Value::String(self.0.to_string())
        }
    }
    impl Serialize for ID {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(&self.0.to_string())
        }
    }
    impl<'de> Deserialize<'de> for ID {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct IDVistor;
            impl<'de> Visitor<'de> for IDVistor {
                type Value = ID;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("uuid")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(ID(u64::from_str(v).expect("Invalid ID")))
                }
            }
            deserializer.deserialize_str(IDVistor)
        }
    }
    pub struct Email(pub String);
    #[Scalar]
    /// email type
    impl ScalarType for Email {
        fn parse(value: Value) -> InputValueResult<Self> {
            if let Value::String(value) = value {
                return Ok(Self(value));
            }
            return Err(InputValueError::custom("not a valid uuid"));
        }

        fn to_value(&self) -> Value {
            Value::String(self.0.to_string())
        }
    }
    pub struct URL(pub String);
    #[Scalar]
    /// representing a URL string
    impl ScalarType for URL {
        fn parse(value: Value) -> InputValueResult<Self> {
            if let Value::String(value) = value {
                if url::Url::parse(&value).is_ok() {
                    return Ok(Self(value));
                }
            }
            return Err(InputValueError::custom("not a valid URL"));
        }

        fn to_value(&self) -> Value {
            Value::String(self.0.to_string())
        }
    }
    impl Serialize for URL {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(&self.0.to_string())
        }
    }
    impl<'de> Deserialize<'de> for URL {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct URLVistor;
            impl<'de> Visitor<'de> for URLVistor {
                type Value = URL;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("uuid")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(URL(v.to_owned()))
                }
            }
            deserializer.deserialize_str(URLVistor)
        }
    }
    pub struct PhoneNumber(pub String);
    #[Scalar]
    /// representing a phone number
    impl ScalarType for PhoneNumber {
        fn parse(value: Value) -> InputValueResult<Self> {
            if let Value::String(value) = value {
                if japhonex::japhonex(japhonex::HyphenCheck::Optional).is_match(&value) {
                    return Ok(Self(value));
                }
            }
            return Err(InputValueError::custom("not a valid Phone number"));
        }
        fn to_value(&self) -> Value {
            Value::String(self.0.to_string())
        }
    }
    impl Serialize for PhoneNumber {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(&self.0.to_string())
        }
    }
    impl<'de> Deserialize<'de> for PhoneNumber {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct PhoneNumberVistor;
            impl<'de> Visitor<'de> for PhoneNumberVistor {
                type Value = PhoneNumber;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("uuid")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(PhoneNumber(v.to_owned()))
                }
            }
            deserializer.deserialize_str(PhoneNumberVistor)
        }
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct TokenData {
        pub id: ID,
        pub username: String,
        pub exp: usize,
    }
    pub struct JWT(pub Option<TokenData>);
    const JWT_SECRET: &'static str = "asifEaifoa@";
    #[Scalar]
    /// A representation for json web token
    impl ScalarType for JWT {
        fn parse(value: Value) -> InputValueResult<Self> {
            if let Value::String(value) = value {
                match jwt_decode(value, JWT_SECRET) {
                    Ok(value) => {
                        return Ok(Self(Some(value)));
                    }
                    Err(err) => {
                        return Err(InputValueError::custom(err));
                    }
                }
            }
            return Err(InputValueError::custom("not a valid jwt string"));
        }
        fn to_value(&self) -> Value {
            match &self.0 {
                Some(data) => return Value::String(jwt_encode(JWT_SECRET, data).unwrap()),
                None => Value::String("".to_owned()),
            }
        }
    }
    impl FromStr for JWT {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match jwt_decode(s.to_string(), JWT_SECRET) {
                Ok(value) => {
                    return Ok(Self(Some(value)));
                }
                Err(err) => {
                    error!("{}", err);
                    return Ok(Self(None));
                }
            }
        }
    }
    #[derive(InputObject, SimpleObject)]
    #[graphql(input_name = "YMOnlyDateRangeInput")]
    /// a time range that only have year and month
    pub struct YMOnlyDateRange {
        pub start_at: YearMonthBundle,
        pub end_at: YearMonthBundle,
    }
    #[derive(InputObject, SimpleObject)]
    #[graphql(input_name = "YMDOnlyDateRangeInput")]
    /// a time range that only have year month and day
    pub struct YMDOnlyDateRange {
        pub start_at: YearMonthDayBundle,
        pub end_at: YearMonthDayBundle,
    }
    #[derive(InputObject, SimpleObject)]
    #[graphql(input_name = "FullTimeRangeInput")]
    /// a full datetime range
    pub struct FullTimeRange {
        pub start_at: DateTimeForGQL,
        pub end_at: DateTimeForGQL,
    }
    #[derive(OneofObject)]
    pub enum TimeRange {
        YearAndMonth(YMOnlyDateRange),
        YearMonthAndDay(YMDOnlyDateRange),
        FullTimeRange(FullTimeRange),
    }
    #[derive(SimpleObject, InputObject)]
    #[graphql(input_name = "CoordinateInput")]
    /// Coordinate type
    pub struct Coordinate {
        pub latitude: Latitude,
        pub longitude: Longitude,
    }
    #[derive(SimpleObject, InputObject)]
    #[graphql(input_name = "LocationBundleInput")]
    /// Location Bundle type
    pub struct LocationBundle {
        pub coordinate: Coordinate,
        pub address: String,
    }
    #[derive(Enum, Copy, Clone, Eq, PartialEq)]
    /// representing the EducationLevel
    pub enum EducationLevel {
        Never,
        PrimarySchool,
        JuniorSchool,
        HighSchool,
        JuniorCollege,
        RegularCollege,
        Postgraduate,
        Doctor,
    }
    #[derive(InputObject)]
    pub struct ResetPassword {
        #[graphql(validator(min_length = 1))]
        verify_code: String,
        #[graphql(validator(min_length = 1))]
        password: String,
    }
}
