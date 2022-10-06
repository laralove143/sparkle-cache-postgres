#![deny(
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction,
    rustdoc::missing_crate_level_docs,
    rustdoc::private_doc_tests,
    rustdoc::invalid_html_tags,
    warnings,
    absolute_paths_not_starting_with_crate,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    non_ascii_idents,
    noop_method_call,
    pointer_structural_match,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    variant_size_differences,
    // Unstable lints:
    // unreachable_pub,
    // Nightly lints:
    // rustdoc::missing_doc_code_examples,
    // fuzzy_provenance_casts,
    // lossy_provenance_casts,
    // must_not_suspend,
    // non_exhaustive_omitted_patterns,
)]
#![allow(
    clippy::blanket_clippy_restriction_lints,
    clippy::exhaustive_structs,
    clippy::missing_inline_in_public_items,
    clippy::implicit_return,
    clippy::unwrap_used,
    clippy::multiple_inherent_impl,
    clippy::pattern_type_mismatch,
    clippy::wildcard_enum_match_arm,
    clippy::exhaustive_enums,
    clippy::self_named_module_files,
    clippy::pub_use,
    clippy::else_if_without_else,
    clippy::std_instead_of_alloc
)]
#![doc = include_str ! ("../README.md")]

use core::ops::Deref;

use sparkle_cache as _;
use sqlx::{query, PgPool};

/// Implementing [`sparkle_cache::Backend`] on [`Cache`]
mod backend;
/// Models for SQL select queries and their conversion to `sparkle_cache` models
pub(crate) mod model;
/// Tests for this crate
#[cfg(test)]
mod tests;

/// The Discord cache
///
/// This is a wrapper over [`PgPool`]
///
/// # Extensibility
///
/// It implements deref to the [`PgPool`] for all of your custom queries
///
/// It's also safe to create your own tables as long as they don't collide with
/// the current table names
///
/// # Indexing
///
/// Most ID and name columns are indexed, you can inspect the database to see
/// which columns are indexed, you can also create your own indexes using the
/// inner [`PgPool`], if you think there's a missing index, please create an
/// issue
#[derive(Debug)]
pub struct Cache(PgPool);

impl Cache {
    /// Create a new cache using the given URL
    ///
    /// Refer to [`sqlx::postgres::PgConnectOptions`] for the URL format
    ///
    /// This sets the tables up to be ready for further queries, it does not
    /// overwrite anything meaning it's okay to run this without cleaning the
    /// database
    ///
    /// # Errors
    ///
    /// Returns the error `SQLx` returns when the database connection failed or
    /// the `init.sql` script failed to run
    pub async fn new(url: &str) -> Result<Self, sqlx::Error> {
        let cache = PgPool::connect(url).await?;

        let init_sql = include_str!("../sql/init.sql");
        for statement in init_sql.split(';') {
            query(statement.trim()).execute(&cache).await?;
        }

        Ok(Self(cache))
    }
}

impl Deref for Cache {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
