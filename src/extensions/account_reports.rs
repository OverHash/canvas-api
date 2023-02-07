use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::CanvasClient;

pub struct Report {
    /// The unique identifier for the report.
    id: u64,
    /// The type of report.
    report: String,
    /// The url to the report download.
    file_url: String,
    /// The attachment api object of the report. Only available after the report
    /// has completed.
    attachment: Option<String>,
    /// The status of the report
    status: String,
    /// The date and time the report was created.
    created_at: String,
    /// The date and time the report started processing.
    started_at: String,
    /// The date and time the report finished processing.
    ended_at: String,
    /// The report parameters.
    parameters: ReportParameters,
    /// The progress of the report
    progress: u8,
    /// This is the current line count being written to the report. It updates
    /// every 1000 records.
    current_line: u64,
}

impl Report {
    /// The unique identifier for the report.
    pub fn id(&self) -> u64 {
        self.id
    }
    /// The type of report.
    pub fn report(&self) -> &str {
        &self.report
    }
    /// The url to the report download.
    pub fn file_url(&self) -> &str {
        &self.file_url
    }
    /// The attachment api object of the report. Only available after the report
    /// has completed.
    pub fn attachment(&self) -> Option<&String> {
        self.attachment.as_ref()
    }
    /// The status of the report
    pub fn status(&self) -> &str {
        &self.status
    }
    /// The date and time the report was created.
    pub fn created_at(&self) -> &str {
        &self.created_at
    }
    /// The date and time the report started processing.
    pub fn started_at(&self) -> &str {
        &self.started_at
    }
    /// The date and time the report finished processing.
    pub fn ended_at(&self) -> &str {
        &self.ended_at
    }
    /// The report parameters.
    pub fn parameters(&self) -> &ReportParameters {
        &self.parameters
    }
    /// The progress of the report
    pub fn progress(&self) -> u8 {
        self.progress
    }
    /// This is the current line count being written to the report. It updates
    /// every 1000 records.
    pub fn current_line(&self) -> u64 {
        self.current_line
    }
}

#[derive(Deserialize, Default)]
pub struct ReportParameters {
    /// The canvas id of the term to get grades from.
    enrollment_term_id: Option<u64>,
    /// If true, deleted objects will be included. If false, deleted objects
    /// will be omitted.
    include_deleted: Option<bool>,
    /// The id of the course to report on.
    course_id: Option<u64>,
    /// The sort order for the csv, Options: 'users', 'courses', 'outcomes'.
    order: Option<String>,
    /// If true, user data will be included. If false, user data will be omitted.
    users: Option<bool>,
    /// If true, account data will be included. If false, account data will be
    /// omitted.
    accounts: Option<bool>,
    /// If true, term data will be included. If false, term data will be omitted.
    terms: Option<bool>,
    /// If true, course data will be included. If false, course data will be omitted.
    courses: Option<bool>,
    /// If true, section data will be included. If false, section data will be
    /// omitted.
    sections: Option<bool>,
    /// If true, enrollment data will be included. If false, enrollment data
    /// will be omitted.
    enrollments: Option<bool>,
    /// If true, group data will be included. If false, group data will be omitted.
    groups: Option<bool>,
    /// If true, data for crosslisted courses will be included. If false, data
    /// for crosslisted courses will be omitted.
    xlist: Option<bool>,
    sis_terms_csv: Option<u64>,
    sis_accounts_csv: Option<u64>,
    /// If true, enrollment state will be included. If false, enrollment state
    /// will be omitted. Defaults to false.
    include_enrollment_state: Option<bool>,
    /// The beginning date for submissions. Max time range is 2 weeks.
    start_at: Option<String>,
    /// The end date for submissions. Max time range is 2 weeks.
    end_at: Option<String>,
}

impl ReportParameters {
    /// The canvas id of the term to get grades from.
    pub fn enrollment_term_id(&self) -> Option<u64> {
        self.enrollment_term_id
    }
    /// If true, deleted objects will be included. If false, deleted objects
    /// will be omitted.
    pub fn include_deleted(&self) -> Option<bool> {
        self.include_deleted
    }
    /// The id of the course to report on.
    pub fn course_id(&self) -> Option<u64> {
        self.course_id
    }
    /// The sort order for the csv, Options: 'users', 'courses', 'outcomes'.
    pub fn order(&self) -> Option<&String> {
        self.order.as_ref()
    }
    /// If true, user data will be included. If false, user data will be omitted.
    pub fn users(&self) -> Option<bool> {
        self.users
    }
    /// If true, account data will be included. If false, account data will be
    /// omitted.
    pub fn accounts(&self) -> Option<bool> {
        self.accounts
    }
    /// If true, term data will be included. If false, term data will be omitted.
    pub fn terms(&self) -> Option<bool> {
        self.terms
    }
    /// If true, course data will be included. If false, course data will be omitted.
    pub fn courses(&self) -> Option<bool> {
        self.courses
    }
    /// If true, section data will be included. If false, section data will be
    /// omitted.
    pub fn sections(&self) -> Option<bool> {
        self.sections
    }
    /// If true, enrollment data will be included. If false, enrollment data
    /// will be omitted.
    pub fn enrollments(&self) -> Option<bool> {
        self.enrollments
    }
    /// If true, group data will be included. If false, group data will be omitted.
    pub fn groups(&self) -> Option<bool> {
        self.groups
    }
    /// If true, data for crosslisted courses will be included. If false, data
    /// for crosslisted courses will be omitted.
    pub fn xlist(&self) -> Option<bool> {
        self.xlist
    }
    pub fn sis_terms_csv(&self) -> Option<u64> {
        self.sis_terms_csv
    }
    pub fn sis_accounts_csv(&self) -> Option<u64> {
        self.sis_accounts_csv
    }
    /// If true, enrollment state will be included. If false, enrollment state
    /// will be omitted. Defaults to false.
    pub fn include_enrollment_state(&self) -> Option<bool> {
        self.include_enrollment_state
    }
    /// The beginning date for submissions. Max time range is 2 weeks.
    pub fn start_at(&self) -> Option<&String> {
        self.start_at.as_ref()
    }
    /// The end date for submissions. Max time range is 2 weeks.
    pub fn end_at(&self) -> Option<&String> {
        self.end_at.as_ref()
    }
}

impl ReportParameters {
    /// Creates a new empty ReportParameters.
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[async_trait]
pub trait AccountReportsExt {
    async fn get_available_reports_by_account(
        &self,
        account_id: u64,
    ) -> Result<GetAvailableReportsByAccountResponse, crate::Error>;

    /// Generates a report instance for the account. Note that `report_type` in the
    /// request must match one of the available report names.
    ///
    /// To fetch a list of available report names and parameters for each
    /// report (including whether or not those parameters are required), see
    /// [`AccountReportsExt::get_available_reports_by_account`].
    ///
    /// A `CreateReportForm` can be created from a `ReportParameters` by using
    /// the `into` method.
    ///
    /// If `skip_message` is true, no message will be sent to the user upon
    /// completion of the report.
    ///
    /// If `users` is true, user data will be included. If false, user data
    /// will be omitted. Note that not every report will respect this field.
    async fn create_report(
        &self,
        account_id: u64,
        report_type: String,
        parameters: CreateReportForm,
    ) -> Result<CreateReportResponse, crate::Error>;

    /// Shows all reports that have been run for the account of a specific type.
    async fn get_reports_by_type(
        &self,
        account_id: u64,
        report_type: String,
    ) -> Result<GetReportsByTypeResponse, crate::Error>;

    /// Returns the status of a report.
    async fn get_report_by_id(
        &self,
        account_id: u64,
        report_type: String,
        report_id: u64,
    ) -> Result<GetReportByIdResponse, crate::Error>;

    /// Deletes a generated report instance.
    async fn delete_report_by_id(
        &self,
        account_id: u64,
        report_type: String,
        report_id: u64,
    ) -> Result<DeleteReportResponse, crate::Error>;
}

// https://canvas.instructure.com/doc/api/account_reports.html#method.account_reports.available_reports
#[derive(Deserialize)]
pub struct ReportResponse {
    pub report: String,
    pub title: String,
    /// The parameters will vary for each report
    pub parameters: Option<serde_json::Value>,
}
type GetAvailableReportsByAccountResponse = Vec<ReportResponse>;

// https://canvas.instructure.com/doc/api/account_reports.html#method.account_reports.create
// CreateReportForm extends from ReportParameters
#[derive(Serialize)]
pub struct CreateReportForm {
    pub enrollment_term_id: Option<u64>,
    pub include_deleted: Option<bool>,
    pub course_id: Option<u64>,
    pub order: Option<String>,
    pub users: Option<bool>,
    pub accounts: Option<bool>,
    pub terms: Option<bool>,
    pub courses: Option<bool>,
    pub sections: Option<bool>,
    pub enrollments: Option<bool>,
    pub groups: Option<bool>,
    pub xlist: Option<bool>,
    pub sis_terms_csv: Option<u64>,
    pub sis_accounts_csv: Option<u64>,
    pub skip_message: Option<bool>,
    pub include_enrollment_state: Option<bool>,
    pub start_at: Option<String>,
    pub end_at: Option<String>,
}
impl From<ReportParameters> for CreateReportForm {
    fn from(value: ReportParameters) -> Self {
        Self {
            accounts: value.accounts,
            course_id: value.course_id,
            courses: value.courses,
            end_at: value.end_at,
            enrollment_term_id: value.enrollment_term_id,
            enrollments: value.enrollments,
            groups: value.groups,
            include_deleted: value.include_deleted,
            include_enrollment_state: value.include_enrollment_state,
            order: value.order,
            sections: value.sections,
            sis_accounts_csv: value.sis_accounts_csv,
            sis_terms_csv: value.sis_terms_csv,
            skip_message: None,
            start_at: value.start_at,
            terms: value.terms,
            users: value.users,
            xlist: value.xlist,
        }
    }
}

type CreateReportResponse = ReportResponse;

// https://canvas.instructure.com/doc/api/account_reports.html#method.account_reports.index
type GetReportsByTypeResponse = Vec<ReportResponse>;

// https://canvas.instructure.com/doc/api/account_reports.html#method.account_reports.show
type GetReportByIdResponse = ReportResponse;

// https://canvas.instructure.com/doc/api/account_reports.html#method.account_reports.destroy
type DeleteReportResponse = ReportResponse;

#[async_trait]
impl AccountReportsExt for CanvasClient {
    async fn get_available_reports_by_account(
        &self,
        account_id: u64,
    ) -> Result<GetAvailableReportsByAccountResponse, crate::Error> {
        let reports = self
            .make_query(&format!("v1/accounts/{account_id}/reports"))
            .send()
            .await?
            .json()
            .await?;

        Ok(reports)
    }

    async fn create_report(
        &self,
        account_id: u64,
        report_type: String,
        parameters: CreateReportForm,
    ) -> Result<CreateReportResponse, crate::Error> {
        let report = self
            .make_post(&format!("v1/accounts/{account_id}/reports/{report_type}"))
            .form(&parameters)
            .send()
            .await?
            .json()
            .await?;

        Ok(report)
    }

    async fn get_reports_by_type(
        &self,
        account_id: u64,
        report_type: String,
    ) -> Result<GetReportsByTypeResponse, crate::Error> {
        let reports = self
            .make_query(&format!("v1/accounts/{account_id}/reports/{report_type}"))
            .send()
            .await?
            .json()
            .await?;

        Ok(reports)
    }

    async fn get_report_by_id(
        &self,
        account_id: u64,
        report_type: String,
        report_id: u64,
    ) -> Result<GetReportByIdResponse, crate::Error> {
        let report = self
            .make_query(&format!(
                "v1/accounts/{account_id}/reports/{report_type}/{report_id}"
            ))
            .send()
            .await?
            .json()
            .await?;

        Ok(report)
    }

    async fn delete_report_by_id(
        &self,
        account_id: u64,
        report_type: String,
        report_id: u64,
    ) -> Result<DeleteReportResponse, crate::Error> {
        let report = self
            .make_delete(&format!(
                "v1/accounts/{account_id}/reports/{report_type}/{report_id}"
            ))
            .send()
            .await?
            .json()
            .await?;

        Ok(report)
    }
}
