mod bitwise_for_parity_check;
mod bool_comparison;
mod breaks;
mod double_comparison;
mod double_parens;
mod duplicate_underscore_args;
mod eq_op;
mod erasing_operations;
mod helpers;
mod ifs;
mod int_operations;
mod loops;
mod manual;
mod panic;
mod performance;
mod single_match;
mod unused_imports;
mod unused_variables;

pub const CRATE_CONFIG: &str = r#"
edition = "2024_07"

[experimental_features]
negative_impls = true
coupons = true
associated_item_constraints = true
"#;
