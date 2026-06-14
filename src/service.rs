//! Service model and admin configuration.
//!
//! A single offering. It belongs to a category, states the problem it
//! solves and the technology focus, and declares how it is engaged
//! (fixed-scope, retainer, or hourly).

use rustio_admin::{Inline, ModelAdmin, RustioAdmin};

#[derive(RustioAdmin)]
pub struct Service {
    pub id: i64,
    pub category_id: i64,
    pub name: String,
    pub slug: String,
    pub summary: String,
    pub problem_solved: String,
    pub tech_focus: String,
    #[rustio(choices = ["fixed", "retainer", "hourly"])]
    pub engagement_model: String,
    pub active: bool,
}

impl ModelAdmin for Service {
    fn list_display() -> &'static [&'static str] {
        &["name", "category_id", "engagement_model", "active"]
    }

    fn list_filter() -> &'static [&'static str] {
        &["engagement_model", "active"]
    }

    fn search_fields() -> &'static [&'static str] {
        &["name", "summary", "tech_focus", "problem_solved"]
    }

    fn ordering() -> &'static [&'static str] {
        &["name"]
    }

    // A service's case studies, listed on its edit page.
    fn inlines() -> &'static [Inline] {
        &[Inline {
            target_model: "CaseStudy",
            fk_field: "service_id",
            label: Some("Case studies"),
            max_rows: 50,
            display_field: Some("title"),
        }]
    }
}
