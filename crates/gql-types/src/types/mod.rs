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
            _ => "unknown error".to_string(),
        })
    }
    use async_graphql::{
        Enum, InputObject, InputValueError, InputValueResult, OneofObject, Scalar, ScalarType,
        SimpleObject, Value,
    };
    use chrono::{prelude::*, LocalResult};
    use serde::{de::Visitor, Deserialize, Serialize};

    /// A type representing Longitude
    pub struct Longitude(pub f64);

    #[Scalar]
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
    /// a type representing a latitude
    pub struct Latitude(pub f64);
    #[Scalar]
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
    /// a type for year and month date values
    /// format like Y-m
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
    /// a type for year month and day date values
    /// format like Y-m-d
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
    /// a full date time type
    /// it is a UTC format one
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
    /// a uuid type
    pub struct UUID(pub uuid::Uuid);
    impl FromStr for UUID {
        type Err = uuid::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self(uuid::Uuid::from_str(s)?))
        }
    }
    #[Scalar]
    impl ScalarType for UUID {
        fn parse(value: Value) -> InputValueResult<Self> {
            if let Value::String(value) = value {
                if let Ok(uuid) = uuid::Uuid::from_str(&value) {
                    return Ok(Self(uuid));
                }
            }
            return Err(InputValueError::custom("not a valid uuid"));
        }

        fn to_value(&self) -> Value {
            Value::String(self.0.to_string())
        }
    }
    impl Serialize for UUID {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(&self.0.to_string())
        }
    }
    impl<'de> Deserialize<'de> for UUID {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct UUIDVistor;
            impl<'de> Visitor<'de> for UUIDVistor {
                type Value = UUID;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("uuid")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(UUID(uuid::Uuid::from_str(v).expect("Invalid UUID")))
                }
            }
            deserializer.deserialize_str(UUIDVistor)
        }
    }
    /// email type
    pub struct Email(pub String);
    #[Scalar]
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
    pub struct PhoneNumber(pub String);
    #[Scalar]
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
    #[derive(Serialize, Deserialize)]
    pub struct TokenData {
        pub id: UUID,
        pub username: String,
    }
    pub struct JWT(pub TokenData);
    const JWT_SECRET: &'static str = "asifEaifoa@";
    #[Scalar]
    impl ScalarType for JWT {
        fn parse(value: Value) -> InputValueResult<Self> {
            if let Value::String(value) = value {
                match jwt_decode(value, JWT_SECRET) {
                    Ok(value) => {
                        return Ok(Self(value));
                    }
                    Err(err) => {
                        return Err(InputValueError::custom(err));
                    }
                }
            }
            return Err(InputValueError::custom("not a valid jwt string"));
        }
        fn to_value(&self) -> Value {
            Value::String(jwt_encode(JWT_SECRET, &self.0).expect("falied to parse data to jwt"))
        }
    }
    /// a time range that only have year and month
    #[derive(InputObject, SimpleObject)]
    #[graphql(input_name = "YMOnlyDateRangeInput")]
    pub struct YMOnlyDateRange {
        pub start_at: YearMonthBundle,
        pub end_at: YearMonthBundle,
    }
    /// a time range that only have year month and day
    #[derive(InputObject, SimpleObject)]
    #[graphql(input_name = "YMDOnlyDateRangeInput")]
    pub struct YMDOnlyDateRange {
        pub start_at: YearMonthDayBundle,
        pub end_at: YearMonthDayBundle,
    }
    /// a full datetime range
    #[derive(InputObject, SimpleObject)]
    #[graphql(input_name = "FullTimeRangeInput")]
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
