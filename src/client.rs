//! Client model and admin configuration.
//!
//! A Swedish company, tracked from first contact (`lead`) through an
//! `active` engagement to a `past` relationship.

use rustio_admin::{Inline, ModelAdmin, RustioAdmin};

#[derive(RustioAdmin)]
pub struct Client {
    pub id: i64,
    pub company_name: String,
    pub org_number: String,
    pub contact_name: String,
    #[rustio(format = "email")]
    pub email: String,
    pub city: String,
    pub industry: String,
    #[rustio(choices = ["lead", "active", "past"])]
    pub status: String,
}

impl ModelAdmin for Client {
    fn list_display() -> &'static [&'static str] {
        &["company_name", "city", "industry", "status"]
    }

    fn list_filter() -> &'static [&'static str] {
        &["status", "industry"]
    }

    fn search_fields() -> &'static [&'static str] {
        &["company_name", "contact_name", "email", "city"]
    }

    fn ordering() -> &'static [&'static str] {
        &["company_name"]
    }

    // A client's engagements, listed on its edit page.
    fn inlines() -> &'static [Inline] {
        &[Inline {
            target_model: "Engagement",
            fk_field: "client_id",
            label: Some("Engagements"),
            max_rows: 50,
            display_field: Some("title"),
        }]
    }
}
