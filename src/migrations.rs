use diesel::backend::Backend;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::BoxError;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run<DB: Backend>(connection: &mut impl MigrationHarness<DB>) -> Result<(), BoxError> {
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}
