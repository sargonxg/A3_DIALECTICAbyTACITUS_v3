//! PostgreSQL store scaffold.
//!
//! This crate will own SQLx repositories, migrations, and transaction policy.

/// Environment variable expected to hold the PostgreSQL connection string.
pub const DATABASE_URL_ENV: &str = "DATABASE_URL";

/// Returns the ordered table families expected by the first migration wave.
pub fn first_migration_families() -> [&'static str; 5] {
    ["capsules", "sources", "claims", "graph", "review_decisions"]
}

#[cfg(test)]
mod tests {
    use super::first_migration_families;

    #[test]
    fn first_migration_tracks_review_data() {
        assert!(first_migration_families().contains(&"review_decisions"));
    }
}
