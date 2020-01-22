// NOTE: this has overwritten the previous `mod.rs`. If this is not what you
// want, simply add `pub mod {{.FullDay}};` at the end of the current `mod.rs`.
// Also: remember to add `pub mod {{.FullYear}}` to `src/lib.rs` for `cargo` to
// actually pick up the `{{.FullYear}}` module as well as the `{{.FullDay}}`
// module.
pub mod {{.FullDay}};
