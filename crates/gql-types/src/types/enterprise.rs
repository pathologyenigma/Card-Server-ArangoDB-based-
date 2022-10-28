use async_graphql::{Enum, InputObject, SimpleObject};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum EnterpriseBusinessNature {
    ForeignVentures,
    ForeignFundedEnterprises,
    PrivateEnterprise,
    StateOwnedEnterprises,
    Extra,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum EnterpriseFinancing {
    NotYet,
    AngelFinancing,
    A,
    B,
    C,
    D,
    Listed,
    NoNeed,
}
#[derive(SimpleObject)]
pub struct Enterprise {
    pub id: super::common::UUID,
    pub complete_name: String,
    pub abbreviation: Option<String>,
    pub business_nature: Option<EnterpriseBusinessNature>,
    pub industry_involving: Option<String>,
    pub enterprise_profile: Option<String>,
    pub enterprise_location: Option<super::common::LocationBundle>,
    pub enterprise_log: Option<super::common::URL>,
    pub enterprise_size: Option<u16>,
    pub tags: Option<Vec<String>>,
    /// a array of json
    /// the json may like this:
    /// {
    /// "attribute_name": "xxxx",
    /// "attribute_value": {
    ///     "type": "image",
    ///     "url": "http://xxxx"
    /// }
    /// }
    pub extra_attributes: Option<Vec<String>>,
    pub tel: Option<String>,
}
#[derive(SimpleObject)]
pub struct Worker {
    pub id: super::common::UUID,
    pub role: String,
    pub auth: String,
    pub name: String,
    pub phone_number: Option<super::common::PhoneNumber>,
}
#[derive(SimpleObject)]
pub struct Job {
    pub id: super::common::UUID,
    pub title: String,
    pub category: String,
    pub detail: String,
    pub min_salary: u32,
    pub max_salary: u32,
    pub required_number: u8,
    pub ontop: bool,
    pub full_time_job: bool,
    pub tags: Option<Vec<String>>,
}
#[derive(SimpleObject)]
pub struct JobInfoDetail {
    pub job: Job,
    pub worker: Worker,
    pub enterprise: Enterprise,
}
#[derive(InputObject)]
pub struct EnterpriseSearchFilter {
    pub size: Option<u16>,
    #[graphql(validator(min_length = 1))]
    pub industry_involving: Option<String>,
    pub enterprise_financing: Option<EnterpriseFinancing>,
    pub business_nature: Option<EnterpriseBusinessNature>,
}
#[derive(InputObject)]
#[graphql(input_name = "WorkerLogInInput")]
pub struct LogInInput {
    phone_number: super::common::PhoneNumber,
    #[graphql(validator(min_length = 1))]
    verify: String,
    #[graphql(validator(min_length = 1))]
    is_verify_code: String,
}

#[derive(InputObject)]
pub struct SearchEnterpriseInput {
    #[graphql(validator(min_length = 1))]
    /// keyword search range:
    /// the name of the enterprise
    /// the enterprise profile
    /// the enterprise tags
    keyword: Option<String>,
    filter: Option<EnterpriseSearchFilter>,
}
#[derive(InputObject)]
pub struct EnterpriseCreateInput {
    pub complete_name: String,
    pub abbreviation: Option<String>,
    pub business_nature: Option<EnterpriseBusinessNature>,
    pub industry_involving: Option<String>,
    pub enterprise_profile: Option<String>,
    pub enterprise_location: Option<super::common::LocationBundle>,
    pub enterprise_log: Option<super::common::URL>,
    pub enterprise_size: Option<u16>,
    pub tags: Option<Vec<String>>,
    /// a array of json
    /// the json may like this:
    /// {
    /// "attribute_name": "xxxx",
    /// "attribute_value": {
    ///     "type": "image",
    ///     "url": "http://xxxx"
    /// }
    /// }
    pub extra_attributes: Option<Vec<String>>,
    pub tel: Option<String>,
}

#[derive(InputObject)]
pub struct EnterpriseEditInput {
    pub id: super::common::UUID,
    pub abbreviation: Option<String>,
    pub business_nature: Option<EnterpriseBusinessNature>,
    pub industry_involving: Option<String>,
    pub enterprise_profile: Option<String>,
    pub enterprise_location: Option<super::common::LocationBundle>,
    pub enterprise_log: Option<super::common::URL>,
    pub enterprise_size: Option<u16>,
    pub tags: Option<Vec<String>>,
    /// a array of json
    /// the json may like this:
    /// {
    /// "attribute_name": "xxxx",
    /// "attribute_value": {
    ///     "type": "image",
    ///     "url": "http://xxxx"
    /// }
    /// }
    pub extra_attributes: Option<Vec<String>>,
    pub tel: Option<String>,
}

#[derive(InputObject)]
pub struct WorkerCreateInput {
    pub role: String,
    pub auth: String,
    pub name: String,
    pub phone_number: super::common::PhoneNumber,
}
#[derive(InputObject)]
pub struct WorkerEditInput {
    pub id: super::common::UUID,
    pub role: Option<String>,
    pub auth: Option<String>,
    pub name: Option<String>,
    pub phone_number: Option<super::common::PhoneNumber>,
}
