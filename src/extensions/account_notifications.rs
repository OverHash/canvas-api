use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::CanvasClient;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountNotification {
    /// The subject of the notifications
    #[serde(rename(serialize = "account_notification[subject]"))]
    subject: String,
    /// The message to be sent in the notification.
    #[serde(rename(serialize = "account_notification[message]"))]
    message: String,
    ///  When to send out the notification.
    ///
    /// For example, `2013-08-28T23:59:00-06:00`
    #[serde(rename(serialize = "account_notification[start_at]"))]
    start_at: String,
    /// When to expire the notification.
    ///
    /// For example, `2013-08-29T23:59:00-06:00`
    #[serde(rename(serialize = "account_notification[end_at]"))]
    end_at: String,
    /// The icon to display with the message.
    ///
    /// Defaults to `warning`.
    #[serde(rename(serialize = "account_notification[icon]"))]
    icon: NotificationIcon,
    /// The roles to send the notification to.
    ///
    /// If [`None`], defaults to all roles.
    #[serde(skip_serializing)]
    role_ids: Option<Vec<u64>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotificationIcon {
    Warning,
    Information,
    Question,
    Error,
    Calendar,
}

impl AccountNotification {
    /// The subject of the notifications
    pub fn subject(&self) -> &str {
        &self.subject
    }
    /// The message to be sent in the notification.
    pub fn message(&self) -> &str {
        &self.message
    }
    ///  When to send out the notification.
    ///
    /// For example, `2013-08-28T23:59:00-06:00`
    pub fn start_at(&self) -> &str {
        &self.start_at
    }
    /// When to expire the notification.
    ///
    /// For example, `2013-08-29T23:59:00-06:00`
    pub fn end_at(&self) -> &str {
        &self.end_at
    }
    /// The icon to display with the message.
    ///
    /// Defaults to `warning`.
    pub fn icon(&self) -> &NotificationIcon {
        &self.icon
    }
    /// The roles to send the notification to.
    ///
    /// If [`None`], defaults to all roles.
    pub fn role_ids(&self) -> Option<&Vec<u64>> {
        self.role_ids.as_ref()
    }
}

pub enum IncludePastNotifications {
    Include,
    Exclude,
}

impl Serialize for IncludePastNotifications {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Include => serializer.serialize_bool(true),
            Self::Exclude => serializer.serialize_bool(false),
        }
    }
}

#[async_trait]
pub trait AccountNotificationsExt {
    /// Returns a list of all global notifications in the account for the
    /// current user.
    ///
    /// Any notifications that have been closed by the user will not be
    /// returned, unless a include_past parameter is passed in as true.
    async fn get_global_notifications_for_account(
        &self,
        account_id: u64,
        include_past: IncludePastNotifications,
    ) -> Result<GetGlobalNotificationsForAccountResponse, crate::Error>;

    /// Returns a global notification for the current user.
    ///
    /// A notification that has been closed by the user will not be returned.
    async fn get_notification_for_account(
        &self,
        account_id: u64,
        notification_id: u64,
    ) -> Result<GetNotificationForAccountByIdResponse, crate::Error>;

    /// If the current user no long wants to see this notification it can be excused with this call.
    ///
    /// Returns the notification before it was closed.
    async fn close_notification_for_account(
        &self,
        account_id: u64,
        notification_id: u64,
    ) -> Result<CloseNotificationForAccountResponse, crate::Error>;

    /// Create and return a new global notification for an account.
    ///
    /// Note that the [`AccountNotification::role_ids`] field will be ignored.
    async fn create_global_notification(
        &self,
        account_id: u64,
        notification: &AccountNotification,
    ) -> Result<CreateGlobalNotificationResponse, crate::Error>;

    // Update global notification for an account.
    ///
    /// Note that the [`AccountNotification::role_ids`] field will be ignored.
    async fn update_global_notification(
        &self,
        account_id: u64,
        notification_id: u64,
        new_notification: &AccountNotification,
    ) -> Result<UpdateGlobalNotificationResponse, crate::Error>;
}

// https://canvas.instructure.com/doc/api/account_notifications.html#method.account_notifications.user_index
type GetGlobalNotificationsForAccountResponse = Vec<AccountNotification>;

// https://canvas.instructure.com/doc/api/account_notifications.html#method.account_notifications.show
type GetNotificationForAccountByIdResponse = AccountNotification;

// https://canvas.instructure.com/doc/api/account_notifications.html#method.account_notifications.user_close_notification
type CloseNotificationForAccountResponse = AccountNotification;

// https://canvas.instructure.com/doc/api/account_notifications.html#method.account_notifications.create
type CreateGlobalNotificationResponse = AccountNotification;

// https://canvas.instructure.com/doc/api/account_notifications.html#method.account_notifications.update
type UpdateGlobalNotificationResponse = AccountNotification;

#[async_trait]
impl AccountNotificationsExt for CanvasClient {
    async fn get_global_notifications_for_account(
        &self,
        account_id: u64,
        include_past: IncludePastNotifications,
    ) -> Result<GetGlobalNotificationsForAccountResponse, crate::Error> {
        let notifications = self
            .make_query(&format!("v1/accounts/{account_id}/account_notifications"))
            .query(&[("include_past", include_past)])
            .send()
            .await?
            .json()
            .await?;

        Ok(notifications)
    }

    async fn get_notification_for_account(
        &self,
        account_id: u64,
        notification_id: u64,
    ) -> Result<GetNotificationForAccountByIdResponse, crate::Error> {
        let notification = self
            .make_query(&format!(
                "v1/accounts/{account_id}/account_notifications/{notification_id}"
            ))
            .send()
            .await?
            .json()
            .await?;

        Ok(notification)
    }

    async fn close_notification_for_account(
        &self,
        account_id: u64,
        notification_id: u64,
    ) -> Result<CloseNotificationForAccountResponse, crate::Error> {
        let notification = self
            .make_delete(&format!(
                "v1/accounts/{account_id}/account_notifications/{notification_id}"
            ))
            .send()
            .await?
            .json()
            .await?;

        Ok(notification)
    }

    async fn create_global_notification(
        &self,
        account_id: u64,
        notification: &AccountNotification,
    ) -> Result<CreateGlobalNotificationResponse, crate::Error> {
        let notification = self
            .make_post(&format!("v1/accounts/{account_id}/account_notifications"))
            .form(notification)
            .send()
            .await?
            .json()
            .await?;

        Ok(notification)
    }

    async fn update_global_notification(
        &self,
        account_id: u64,
        notification_id: u64,
        new_notification: &AccountNotification,
    ) -> Result<UpdateGlobalNotificationResponse, crate::Error> {
        let notification = self
            .make_put(&format!(
                "v1/accounts/{account_id}/account_notifications/{notification_id}"
            ))
            .form(new_notification)
            .send()
            .await?
            .json()
            .await?;

        Ok(notification)
    }
}
