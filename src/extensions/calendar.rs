use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::CanvasClient;

#[derive(Deserialize, Debug)]
pub struct AccountCalendar {
    /// The ID of the account associated with this calendar.
    id: u64,
    /// The name of the account associated with this calendar.
    name: String,
    /// The account's parent ID. `None` if this is the root account.
    parent_account_id: Option<u64>,
    /// The ID of the root account. `None` if this is the root account.
    root_account_id: Option<u64>,
    /// Whether this calendar is visible to users.
    visible: bool,
    /// Number of this account's direct sub-accounts.
    sub_account_count: u64,
    /// Asset string of the account.
    asset_string: String,
    /// URL to get full detailed events.
    calendar_event_url: String,
    /// Whether the user can create calendar events
    can_create_calendar_events: bool,
    /// API path to create events for the account.
    create_calendar_event_url: String,
    /// URL to open the more options event editor.
    new_calendar_event_url: String,
}

impl AccountCalendar {
    /// The ID of the account associated with this calendar.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// The name of the account associated with this calendar.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The account's parent ID. `None` if this is the root account.
    pub fn parent_account_id(&self) -> Option<u64> {
        self.parent_account_id
    }

    /// The ID of the root account. `None` if this is the root account.
    pub fn root_account_id(&self) -> Option<u64> {
        self.root_account_id
    }

    /// Whether this calendar is visible to users.
    pub fn visible(&self) -> bool {
        self.visible
    }

    /// Number of this account's direct sub-accounts.
    pub fn sub_account_count(&self) -> u64 {
        self.sub_account_count
    }

    /// Asset string of the account.
    pub fn asset_string(&self) -> &str {
        &self.asset_string
    }

    /// URL to get full detailed events.
    pub fn calendar_event_url(&self) -> &str {
        &self.calendar_event_url
    }

    /// Whether the user can create calendar events
    pub fn can_create_calendar_events(&self) -> bool {
        self.can_create_calendar_events
    }

    /// API path to create events for the account.
    pub fn create_calendar_event_url(&self) -> &str {
        &self.create_calendar_event_url
    }

    /// URL to open the more options event editor.
    pub fn new_calendar_event_url(&self) -> &str {
        &self.new_calendar_event_url
    }
}

#[derive(Serialize)]
pub struct AccountVisibility {
    /// The account's id.
    id: u64,
    #[serde(rename = "visible")]
    /// Indicates the visibility of the account.
    visibility: Visibility,
}

impl AccountVisibility {
    /// For use in `CalendarExt::set_multiple_account_calendar_visible`.
    ///
    /// `id` is the account's id, while `visible` determines whether the account calendar is visible.
    pub fn new(id: u64, visibility: Visibility) -> AccountVisibility {
        Self { id, visibility }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Visible,
    Hidden,
}

impl Visibility {
    fn as_bool(&self) -> bool {
        match self {
            Visibility::Visible => true,
            Visibility::Hidden => false,
        }
    }
}

#[async_trait]
pub trait CalendarExt {
    /// Returns a paginated list of account calendars available to the current user.
    ///
    /// Includes visible account calendars where the user has an account association.
    ///
    /// To search calendars that have a specific search term, see [`CalenderExt::search_calendars`].
    ///
    /// [See docs](https://canvas.instructure.com/doc/api/account_calendars.html#method.account_calendars_api.index).
    async fn all_calendars(&self) -> Result<Vec<AccountCalendar>, crate::Error>;

    /// Returns a paginated list of account calendars available to the current user that match the specified search term.
    ///
    /// Includes visible account calendars where the user has an account association.
    ///
    /// Searches available account calendars for the specified term. Term must be at least 2 characters.
    ///
    /// To search all calendars without the search term, see [`CalenderExt::all_calendars`].
    ///
    /// [See docs](https://canvas.instructure.com/doc/api/account_calendars.html#method.account_calendars_api.index).
    async fn search_calendars(
        &self,
        search_term: &str,
    ) -> Result<Vec<AccountCalendar>, crate::Error>;

    /// Get details about a specific account calendar.
    ///
    /// [See docs](https://canvas.instructure.com/doc/api/account_calendars.html#method.account_calendars_api.show).
    async fn calendar_by_account_id(
        &self,
        account_id: u64,
    ) -> Result<AccountCalendar, crate::Error>;

    /// Set an account calendar as hidden or visible.
    ///
    /// Requires the `manage_account_calendar_visibility` permission on the account.
    ///
    /// Returns the new [`AccountCalendar`].
    ///
    /// [See docs](https://canvas.instructure.com/doc/api/account_calendars.html#method.account_calendars_api.update).
    async fn set_account_calendar_visible(
        &self,
        account_id: u64,
        visibility: Visibility,
    ) -> Result<AccountCalendar, crate::Error>;

    /// Set visibility on many calendars simultaneously. Requires the `manage_account_calendar_visibility` permission on the account.
    ///
    /// [See docs](https://canvas.instructure.com/doc/api/account_calendars.html#method.account_calendars_api.bulk_update)
    async fn set_many_account_calendars_visible(
        &self,
        account_id: u64,
        account_calendars: &[AccountVisibility],
    ) -> Result<AccountCalendar, crate::Error>;

    /// Returns a paginated list of account calendars for the provided account and its first level of sub-accounts.
    ///
    /// `filter` determines the type of accounts that will be included in the search.
    ///
    /// Requires the `manage_account_calendar_visibility` permission.
    ///
    /// To search all account calendars that match a search term, see [`CalendarExt::all_account_calendars`].
    ///
    /// [See docs](https://canvas.instructure.com/doc/api/account_calendars.html#method.account_calendars_api.all_calendars).
    async fn all_account_calendars(
        &self,
        account_id: u64,
        filter: Visibility,
    ) -> Result<Vec<AccountCalendar>, crate::Error>;

    /// Returns a paginated list of account calendars for the provided account and its first level of sub-accounts that matches the specified `search_term`.
    ///
    /// `filter` determines the type of accounts that will be included in the search.
    ///
    /// `search_term` is a term greater than two characters which accounts must match.
    ///
    /// Requires the `manage_account_calendar_visibility` permission.
    ///
    /// To search all account calendars without the search term, see [`CalendarExt::all_account_calendars`].
    ///
    /// [See docs](https://canvas.instructure.com/doc/api/account_calendars.html#method.account_calendars_api.all_calendars).
    async fn search_all_account_calendars(
        &self,
        account_id: u64,
        search_term: &str,
        filter: Visibility,
    ) -> Result<Vec<AccountCalendar>, crate::Error>;

    /// Returns the number of visible account calendars.
    ///
    /// [See docs](https://canvas.instructure.com/doc/api/account_calendars.html#method.account_calendars_api.visible_calendars_count).
    async fn count_account_visible_calendars(&self, account_id: u64) -> Result<u64, crate::Error>;
}

// https://canvas.instructure.com/doc/api/account_calendars.html#method.account_calendars_api.index
#[derive(Deserialize)]
struct AccountCalendarsResponse {
    account_calendars: Vec<AccountCalendar>,
}
// https://canvas.instructure.com/doc/api/all_resources.html#method.account_calendars_api.show
type AccountCalendarResponse = AccountCalendar;
// https://canvas.instructure.com/doc/api/all_resources.html#method.account_calendars_api.update
type SetAccountCalendarVisibleResponse = AccountCalendar;
// https://canvas.instructure.com/doc/api/account_calendars.html#method.account_calendars_api.bulk_update
type SetManyAccountCalendarsVisibleResponse = AccountCalendar;

// https://canvas.instructure.com/doc/api/account_calendars.html#method.account_calendars_api.all_calendars
#[derive(Deserialize)]
struct AllAccountCalendarsResponse {
    account_calendars: Vec<AccountCalendar>,
}

// https://canvas.instructure.com/doc/api/account_calendars.html#method.account_calendars_api.visible_calendars_count
#[derive(Deserialize)]
struct CountAccountVisibleCalendarsResponse {
    count: u64,
}

#[async_trait]
impl CalendarExt for CanvasClient {
    async fn all_calendars(&self) -> Result<Vec<AccountCalendar>, crate::Error> {
        let accounts: AccountCalendarsResponse = self
            .make_query("v1/account_calendars")
            .send()
            .await?
            .json()
            .await?;

        Ok(accounts.account_calendars)
    }

    async fn search_calendars(
        &self,
        search_term: &str,
    ) -> Result<Vec<AccountCalendar>, crate::Error> {
        let accounts: AccountCalendarsResponse = self
            .make_query("v1/account_calendars")
            .query(&[("search_term", search_term)])
            .send()
            .await?
            .json()
            .await?;

        Ok(accounts.account_calendars)
    }

    async fn calendar_by_account_id(
        &self,
        account_id: u64,
    ) -> Result<AccountCalendar, crate::Error> {
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
        visible: Visibility,
    ) -> Result<AccountCalendar, crate::Error> {
        let account: SetAccountCalendarVisibleResponse = self
            .make_put(&format!("v1/account_calendar/{account_id}"))
            .form(&[("visible", visible.as_bool())])
            .send()
            .await?
            .json()
            .await?;

        Ok(account)
    }

    async fn set_many_account_calendars_visible(
        &self,
        account_id: u64,
        account_calendars: &[AccountVisibility],
    ) -> Result<AccountCalendar, crate::Error> {
        let account: SetManyAccountCalendarsVisibleResponse = self
            .make_put(&format!("v1/accounts/{account_id}/account_calendars"))
            .form(account_calendars)
            .send()
            .await?
            .json()
            .await?;

        Ok(account)
    }

    async fn all_account_calendars(
        &self,
        account_id: u64,
        filter: Visibility,
    ) -> Result<Vec<AccountCalendar>, crate::Error> {
        let accounts: AllAccountCalendarsResponse = self
            .make_query(&format!("v1/accounts/{account_id}/account_calendars"))
            .query(&[("filter", filter)])
            .send()
            .await?
            .json()
            .await?;

        Ok(accounts.account_calendars)
    }

    async fn search_all_account_calendars(
        &self,
        account_id: u64,
        search_term: &str,
        filter: Visibility,
    ) -> Result<Vec<AccountCalendar>, crate::Error> {
        let accounts: AllAccountCalendarsResponse = self
            .make_query(&format!("v1/accounts/{account_id}/account_calendars"))
            .query(&[("search_term", search_term)])
            .query(&[("filter", filter)])
            .send()
            .await?
            .json()
            .await?;

        Ok(accounts.account_calendars)
    }

    async fn count_account_visible_calendars(&self, account_id: u64) -> Result<u64, crate::Error> {
        let count: CountAccountVisibleCalendarsResponse = self
            .make_query(&format!("v1/accounts/{account_id}/visible_calendars_count"))
            .send()
            .await?
            .json()
            .await?;

        Ok(count.count)
    }
}
