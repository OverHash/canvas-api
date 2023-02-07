mod canvas_client;
mod error;
pub mod extensions;

pub use canvas_client::CanvasClient;
pub use error::Error;

// api extensions
pub use extensions::{
    account_domains::AccountDomainsExt, account_notifications::AccountNotificationsExt,
    calendar::CalendarExt,
};
