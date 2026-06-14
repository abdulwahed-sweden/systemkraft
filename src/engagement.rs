//! Engagement model and admin configuration.
//!
//! A concrete piece of work delivered for one client under one service,
//! moving from `proposed` to `in_progress` to `delivered`.

use rustio_admin::{DateTime, ModelAdmin, RustioAdmin, Utc};

#[derive(RustioAdmin)]
pub struct Engagement {
    pub id: i64,
    pub client_id: i64,
    pub service_id: i64,
    pub title: String,
    #[rustio(choices = ["proposed", "in_progress", "delivered"])]
    pub status: String,
    pub started: DateTime<Utc>,
}

impl ModelAdmin for Engagement {
    fn list_display() -> &'static [&'static str] {
        &["title", "client_id", "service_id", "status", "started"]
    }

    fn list_filter() -> &'static [&'static str] {
        &["status"]
    }

    fn search_fields() -> &'static [&'static str] {
        &["title"]
    }

    fn ordering() -> &'static [&'static str] {
        &["-started"]
    }
}
