use pyo3::prelude::*;

mod bookmark;
mod community;
mod geo;
mod user;
mod utils;
mod tweet;
mod client;
mod media;
mod streaming;
mod notification;

use bookmark::Bookmark;
use community::Community;
use geo::Geo;
use user::User;
use tweet::Tweet;
use client::Client;
use media::Media;
use streaming::Stream;
use notification::NotificationManager;

/// Python module definition
#[pymodule]
fn twikit(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Bookmark>()?;
    m.add_class::<Community>()?;
    m.add_class::<Geo>()?;
    m.add_class::<User>()?;
    m.add_class::<Tweet>()?;
    m.add_class::<Client>()?;
    m.add_class::<Media>()?;
    m.add_class::<Stream>()?;
    m.add_class::<NotificationManager>()?;
    Ok(())
}
