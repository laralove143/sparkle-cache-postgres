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
    unreachable_pub,
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
    clippy::else_if_without_else
)]
#![doc = include_str!("../README.md")]

use core::ops::Deref;

use sqlx::PgPool;

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
#[derive(Debug)]
pub struct Cache(PgPool);

impl Deref for Cache {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
