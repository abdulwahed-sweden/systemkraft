//! Inquiry model and admin configuration.
//!
//! An inbound contact submission from a prospective client. Captured so
//! every lead lands in the same audited place as the rest of the CRM.

use rustio_admin::{DateTime, ModelAdmin, RustioAdmin, Utc};

#[derive(RustioAdmin)]
pub struct Inquiry {
    pub id: i64,
    pub company: String,
    #[rustio(format = "email")]
    pub email: String,
    pub service_interest: String,
    pub message: String,
    pub created: DateTime<Utc>,
}

impl ModelAdmin for Inquiry {
    fn list_display() -> &'static [&'static str] {
        &["company", "email", "service_interest", "created"]
    }

    fn list_filter() -> &'static [&'static str] {
        &["service_interest"]
    }

    fn search_fields() -> &'static [&'static str] {
        &["company", "email", "message"]
    }

    fn ordering() -> &'static [&'static str] {
        &["-created"]
    }
}
