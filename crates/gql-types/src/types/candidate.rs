use async_graphql::{Enum, SimpleObject, InputObject, OneofObject};
use serde::{Serialize, Deserialize};

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum CandidateJobStatus {
    NoJobButNoJob,
    NoJobButWantJob,
    OnTheJob,
    OnTheJobButLookingForAJob,
    GraduatingStudent,
}
#[derive(SimpleObject, Serialize, Deserialize)]
pub struct JobExpectation {
    pub id: super::common::UUID,
    pub category: String,
    pub industry: String,
    pub city: String,
    pub salary_min: u32,
    pub salary_max: Option<u32>,
}
#[derive(SimpleObject, Serialize, Deserialize)]
pub struct Candidate {
    pub status: CandidateJobStatus,
    pub current_city: String,
    pub first_time_working: super::common::YearMonthBundle,
    pub job_expectations: Vec<JobExpectation>,
    pub last_updated_at: super::common::DateTimeForGQL,
}
#[derive(InputObject)]
pub struct JobExpectationCreateInput {
    #[graphql(validator(min_length = 1))]
    pub category: String,
    #[graphql(validator(min_length = 1))]
    pub industry: String,
    #[graphql(validator(min_length = 1))]
    pub city: String,
    pub salary_min: u32,
    pub salary_max: Option<u32>,
}
#[derive(InputObject)]
pub struct JobExpectationEditInput {
    pub id: super::common::UUID,
    #[graphql(validator(min_length = 1))]
    pub category: Option<String>,
    #[graphql(validator(min_length = 1))]
    pub industry: Option<String>,
    #[graphql(validator(min_length = 1))]
    pub city: Option<String>,
    pub salary_min: Option<u32>,
    pub salary_max: Option<u32>,
}
#[derive(OneofObject)]
pub enum JobExpectationInput {
    Create(JobExpectationCreateInput),
    Edit(JobExpectationEditInput),
    Delete(super::common::UUID)
}

#[derive(InputObject)]
pub struct CandidateCreateInput {
    pub status: CandidateJobStatus,
    #[graphql(validator(min_length = 1))]
    pub current_city: String,
    pub first_time_working: super::common::YearMonthBundle,
    pub job_expectations: Vec<JobExpectationCreateInput>,
    pub last_updated_at: super::common::DateTimeForGQL,
}
#[derive(InputObject)]
pub struct CandidateEditInput {
    pub status: CandidateJobStatus,
    #[graphql(validator(min_length = 1))]
    pub current_city: String,
    pub first_time_working: super::common::YearMonthBundle,
    pub job_expectations: Vec<JobExpectationInput>,
    pub last_updated_at: super::common::DateTimeForGQL,
}
#[derive(SimpleObject)]
pub struct ResumeEducationExp {
    pub id: super::common::UUID,
    pub institution_name: String,
    pub education_level: super::common::EducationLevel,
    pub full_time: bool,
    pub major: String,
    pub time: super::common::YMOnlyDateRange,
    pub performance: String,
}
#[derive(InputObject)]
pub struct ResumeEducationExpCreateInput {
    #[graphql(validator(min_length = 1))]
    pub institution_name: String,
    pub education_level: super::common::EducationLevel,
    pub full_time: bool,
    #[graphql(validator(min_length = 1))]
    pub major: String,
    pub time: super::common::YMOnlyDateRange,
    #[graphql(validator(min_length = 1))]
    pub performance: String,
}
#[derive(InputObject)]
pub struct ResumeEducationExpEditInput {
    pub id: super::common::UUID,
    #[graphql(validator(min_length = 1))]
    pub institution_name: Option<String>,
    pub education_level: Option<super::common::EducationLevel>,
    pub full_time: Option<bool>,
    #[graphql(validator(min_length = 1))]
    pub major: Option<String>,
    pub time: Option<super::common::YMOnlyDateRange>,
    #[graphql(validator(min_length = 1))]
    pub performance: Option<String>,
}
#[derive(OneofObject)]
pub enum ResumeEducationExpInput {
    Create(ResumeEducationExpCreateInput),
    Edit(ResumeEducationExpEditInput),
    Delete(super::common::UUID),
}
#[derive(SimpleObject)]
pub struct ResumeWorkExp {
    id: super::common::UUID,
    company_name: String,
    role: String,
    department_name: String,
    time: super::common::YMOnlyDateRange,
    detail: String,
}
#[derive(InputObject)]
pub struct ResumeWorkExpCreateInput {
    #[graphql(validator(min_length = 1))]
    company_name: String,
    #[graphql(validator(min_length = 1))]
    role: String,
    #[graphql(validator(min_length = 1))]
    department_name: String,
    time: super::common::YMOnlyDateRange,
    #[graphql(validator(min_length = 1))]
    detail: String,
}
#[derive(InputObject)]
pub struct ResumeWorkExpEditInput {
    id: super::common::UUID,
    #[graphql(validator(min_length = 1))]
    company_name: Option<String>,
    #[graphql(validator(min_length = 1))]
    role: Option<String>,
    #[graphql(validator(min_length = 1))]
    department_name: Option<String>,
    time: Option<super::common::YMOnlyDateRange>,
    #[graphql(validator(min_length = 1))]
    detail: Option<String>,
}
#[derive(OneofObject)]
pub enum ResumeWorkExpInput {
    Create(ResumeWorkExpCreateInput),
    Edit(ResumeWorkExpEditInput),
    Delete(super::common::UUID),
}
#[derive(SimpleObject)]
pub struct ResumeProjectExp {
    pub id: super::common::UUID,
    pub project_name: String,
    pub role: String,
    pub time: super::common::YMOnlyDateRange,
    pub description: String,
    pub performance: String
}
#[derive(InputObject)]
pub struct ResumeProjectExpCreateInput {
    #[graphql(validator(min_length = 1))]
    pub project_name: String,
    #[graphql(validator(min_length = 1))]
    pub role: String,
    pub time: super::common::YMOnlyDateRange,
    #[graphql(validator(min_length = 1))]
    pub description: String,
    #[graphql(validator(min_length = 1))]
    pub performance: String
}
#[derive(InputObject)]
pub struct ResumeProjectExpEditInput {
    pub id: super::common::UUID,
    #[graphql(validator(min_length = 1))]
    pub project_name: Option<String>,
    #[graphql(validator(min_length = 1))]
    pub role: Option<String>,
    pub time: Option<super::common::YMOnlyDateRange>,
    #[graphql(validator(min_length = 1))]
    pub description: Option<String>,
    #[graphql(validator(min_length = 1))]
    pub performance: Option<String>,
}
#[derive(OneofObject)]
pub enum ResumeProjectExpInput {
    Create(ResumeProjectExpCreateInput),
    Edit(ResumeProjectExpEditInput),
    Delete(super::common::UUID),
}
#[derive(SimpleObject)]
pub struct OnlineResume {
    pub title: String,
    pub skill: Vec<String>,
    pub personal_advantages: String,
    pub education_experiences: Vec<ResumeEducationExp>,
    pub work_experiences: Vec<ResumeWorkExp>,
    pub project_experiences: Vec<ResumeProjectExp>,
}
#[derive(InputObject)]
pub struct OnlineResumeCreateInput {
    #[graphql(validator(min_length = 1))]
    pub title: String,
    #[graphql(validator(min_items = 1, list, min_length = 1))]
    pub skill: Vec<String>,
    #[graphql(validator(min_length = 1))]
    pub personal_advantages: String,
    #[graphql(validator(min_items = 1))]
    pub education_experiences: Option<Vec<ResumeEducationExpCreateInput>>,
    #[graphql(validator(min_items = 1))]
    pub work_experiences: Option<Vec<ResumeWorkExpCreateInput>>,
    #[graphql(validator(min_items = 1))]
    pub project_experiences: Option<Vec<ResumeProjectExpCreateInput>>,
}
#[derive(InputObject)]
pub struct OnlineResumeEditInput {
    /// this id is the id of the job_expectation
    /// that binded with this resume
    pub id: super::common::UUID,
    #[graphql(validator(min_length = 1))]
    pub title: Option<String>,
    #[graphql(validator(min_items = 1, list, min_length = 1))]
    pub skill: Option<Vec<String>>,
    #[graphql(validator(min_length = 1))]
    pub personal_advantages: Option<String>,
    #[graphql(validator(min_items = 1))]
    pub education_experiences: Option<Vec<ResumeEducationExpInput>>,
    #[graphql(validator(min_items = 1))]
    pub work_experiences: Option<Vec<ResumeWorkExpInput>>,
    #[graphql(validator(min_items = 1))]
    pub project_experiences: Option<Vec<ResumeProjectExpInput>>,
}