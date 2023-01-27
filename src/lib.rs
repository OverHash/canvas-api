mod calendar;
mod canvas_client;
mod error;

pub use canvas_client::CanvasClient;
pub use error::Error;

// api extensions
pub use calendar::{AccountCalendar, CalendarExt};
