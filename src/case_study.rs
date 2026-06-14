//! CaseStudy model and admin configuration.
//!
//! The proof behind a service — a problem that was solved, the approach
//! taken, and the measurable outcome. `published` gates whether it is
//! counted on the public homepage.

use rustio_admin::{ModelAdmin, RustioAdmin};

#[derive(RustioAdmin)]
pub struct CaseStudy {
    pub id: i64,
    pub service_id: i64,
    pub title: String,
    pub problem: String,
    pub solution: String,
    pub outcome: String,
    pub published: bool,
}

impl ModelAdmin for CaseStudy {
    fn list_display() -> &'static [&'static str] {
        &["title", "service_id", "published"]
    }

    fn list_filter() -> &'static [&'static str] {
        &["published"]
    }

    fn search_fields() -> &'static [&'static str] {
        &["title", "problem", "solution", "outcome"]
    }

    fn ordering() -> &'static [&'static str] {
        &["-id"]
    }
}
