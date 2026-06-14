//! ServiceCategory model and admin configuration.
//!
//! The top-level grouping of the work offered: Rust admin platforms,
//! security audits, robotics & systems engineering, secure web apps.

use rustio_admin::{Inline, ModelAdmin, RustioAdmin};

#[derive(RustioAdmin)]
pub struct ServiceCategory {
    pub id: i64,
    pub name: String,
    pub slug: String,
}

impl ModelAdmin for ServiceCategory {
    fn list_display() -> &'static [&'static str] {
        &["name", "slug"]
    }

    fn list_filter() -> &'static [&'static str] {
        &[]
    }

    fn search_fields() -> &'static [&'static str] {
        &["name", "slug"]
    }

    fn ordering() -> &'static [&'static str] {
        &["name"]
    }

    // The services that belong to this category, listed on its edit page.
    fn inlines() -> &'static [Inline] {
        &[Inline {
            target_model: "Service",
            fk_field: "category_id",
            label: Some("Services"),
            max_rows: 50,
            display_field: Some("name"),
        }]
    }
}
