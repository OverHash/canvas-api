use crate::CanvasClient;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct AccountCalendar {
    /// The ID of the account associated with this calendar.
    pub id: u64,
    /// The name of the account associated with this calendar
    pub name: String,
    /// The account's parent ID. `None` if this is the root account.
    pub parent_account_id: Option<u64>,
    /// The ID of the root account. `None` if this is the root account.
    pub root_account_id: Option<u64>,
    /// Whether this calendar is visible to users.
    pub visible: bool,
    /// Number of this account's direct sub-accounts.
    pub sub_account_count: u64,
    /// Asset string of the account.
    pub asset_string: String,
    /// URL to get full detailed events.
    pub calendar_event_url: String,
    /// Whether the user can create calendar events
    pub can_create_calendar_events: bool,
    /// API path to create events for the account.
    pub create_calendar_event_url: String,
    /// URL to open the more options event editor.
    pub new_calendar_event_url: String,
}

#[derive(Serialize)]
pub struct AccountVisibility {
    /// The account's id.
    pub id: u64,
    /// A boolean indicating whether the account calendar is visible.
    pub visible: bool,
}

impl AccountVisibility {
    /// For use in `CalendarExt::set_multiple_account_calendar_visible`.
    ///
    /// `id` is the account's id, while `visible` determines whether the account calendar is visible.
    pub fn new(id: u64, visible: bool) -> AccountVisibility {
        Self { id, visible }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchFilter {
    Visible,
    Hidden,
}

#[async_trait]
pub trait CalendarExt {
    /// Returns a paginated list of account calendars available to the current user.
    ///
    /// Includes visible account calendars where the user has an account association.
    ///
    /// An optional search term can be included (min 2 characters).
    ///
    /// [See docs](https://canvas.instructure.com/doc/api/all_resources.html#method.account_calendars_api.index).
    async fn account_calendars(
        &self,
        search_term: Option<String>,
    ) -> Result<Vec<AccountCalendar>, crate::Error>;

    /// Get details about a specific account calendar.
    ///
    /// [See docs](https://canvas.instructure.com/doc/api/all_resources.html#method.account_calendars_api.show).
    async fn account_calendar(&self, account_id: u64) -> Result<AccountCalendar, crate::Error>;

    /// Set an account calendar as hidden or visible.
    ///
    /// Requires the `manage_account_calendar_visibility` permission on the account.
    ///
    /// Returns the new [`AccountCalendar`].
    ///
    /// [See docs](https://canvas.instructure.com/doc/api/all_resources.html#method.account_calendars_api.update).
    async fn set_account_calendar_visible(
        &self,
        account_id: u64,
        visible: bool,
    ) -> Result<AccountCalendar, crate::Error>;

    /// Set visibility on many calendars simultaneously. Requires the `manage_account_calendar_visibility` permission on the account.
    ///
    /// [See docs](https://canvas.instructure.com/doc/api/account_calendars.html#method.account_calendars_api.bulk_update)
    async fn set_multiple_account_calendars_visible(
        &self,
        account_id: u64,
        account_calendars: &[AccountVisibility],
    ) -> Result<AccountCalendar, crate::Error>;

    /// Returns a paginated list of account calendars for the provided account and its first level of sub-accounts.
    ///
    /// Includes hidden calendars in the response.
    ///
    /// Requires the `manage_account_calendar_visibility` permission.
    ///
    /// [See docs](https://canvas.instructure.com/doc/api/account_calendars.html#method.account_calendars_api.all_calendars).
    async fn all_account_calendars(
        &self,
        account_id: u64,
        search_term: String,
        filter: SearchFilter,
    ) -> Result<Vec<AccountCalendar>, crate::Error>;
}

// https://canvas.instructure.com/doc/api/all_resources.html#method.account_calendars_api.index
#[derive(Deserialize)]
struct AccountCalendarsResponse {
    account_calendars: Vec<AccountCalendar>,
}
// https://canvas.instructure.com/doc/api/all_resources.html#method.account_calendars_api.show
type AccountCalendarResponse = AccountCalendar;
// https://canvas.instructure.com/doc/api/all_resources.html#method.account_calendars_api.update
type SetAccountCalendarVisibleResponse = AccountCalendar;
// https://canvas.instructure.com/doc/api/account_calendars.html#method.account_calendars_api.bulk_update
type UpdateManyCalendarsVisibilityResponse = AccountCalendar;

// https://canvas.instructure.com/doc/api/account_calendars.html#method.account_calendars_api.all_calendars
#[derive(Deserialize)]
struct ListAllAccountCalendarsResponse {
	account_calendars: Vec<AccountCalendar>,
}

#[async_trait]
impl CalendarExt for CanvasClient {
    async fn account_calendars(
        &self,
        search_term: Option<String>,
    ) -> Result<Vec<AccountCalendar>, crate::Error> {
        let accounts: AccountCalendarsResponse = self
            .make_query("v1/account_calendars")
            .query(&match search_term {
                Some(term) => vec![("search_term", term)],
                None => vec![],
            })
            .send()
            .await?
            .json()
            .await?;

        Ok(accounts.account_calendars)
    }

    async fn account_calendar(&self, account_id: u64) -> Result<AccountCalendar, crate::Error> {
        let account: AccountCalendarResponse = self
            .make_query(&format!("v1/account_calendar/{account_id}"))
            .send()
            .await?
            .json()
            .await?;

        Ok(account)
    }

    async fn set_account_calendar_visible(
        &self,
        account_id: u64,
        visible: bool,
    ) -> Result<AccountCalendar, crate::Error> {
        let account: SetAccountCalendarVisibleResponse = self
            .make_put(&format!("v1/account_calendar/{account_id}"))
            .form(&[("visible", visible)])
            .send()
            .await?
            .json()
            .await?;

        Ok(account)
    }

    async fn set_multiple_account_calendars_visible(
        &self,
        account_id: u64,
        account_calendars: &[AccountVisibility],
    ) -> Result<AccountCalendar, crate::Error> {
        let account: UpdateManyCalendarsVisibilityResponse = self
            .make_put(&format!("v1/accounts/{account_id}/account_calendars"))
            .form(account_calendars)
            .send()
            .await?
            .json()
            .await?;

        Ok(account)
    }

	async fn all_account_calendars(&self, account_id: u64, search_filter: Option<String>, filter: Option<SearchFilter>) -> Result<Vec<AccountCalendar>, crate::Error> {
		let accounts: ListAllAccountCalendarsResponse = self.make_query(format!("v1/accounts/{account_id}/account_calendars")).query(&[])
	}
}
