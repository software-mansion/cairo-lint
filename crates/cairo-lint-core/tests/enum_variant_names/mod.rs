use crate::{test_lint_diagnostics, test_lint_fixer};

const ENUM_WITH_SUFFIXED_NAME: &str = r#"
enum Cake {
   BlackForestCake,
   HummingbirdCake,
   BattenbergCake,
}
"#;

const ENUM_WITH_PREFIXED_NAME: &str = r#"
enum Cake {
   CakeBlackForest,
   CakeHummingbird,
   CakeBattenberg,
}
"#;

const ENUM_WITH_PREFIX: &str = r#"
enum Wood {
    BlackForest,
    WhiteForest,
}
"#;

const ENUM_WITH_SUFFIX: &str = r#"
enum Wood {
    ForestBlack,
    ForestWhite,
}
"#;

const ENUM_WITH_NOT_ALL_PREFIXED: &str = r#"
enum Cake {
   CakeBlackForest,
   CakeCakeHummingbird,
   Battenberg,
}
"#;

const ENUM_WITH_NOT_ALL_SUFFIXED: &str = r#"
enum Cake {
   BlackForestCake,
   HummingbirdCake,
   Battenberg,
}
"#;

const ENUM_WITH_NOT_CAMEL_CASE_SUFFIX: &str = r#"
enum Cake {
   BlackForestcake,
   Hummingbirdcake,
   Battenbergcake,
}
"#;

const ENUM_SINGLE: &str = r#"
enum Cake {
   BlackForest,
}
"#;

const ENUM_WITH_NOT_CAMEL_CASE_PREFIX: &str = r#"
enum Cake {
   CakeblackForest,
   Cakehummingbird,
   Cakebattenberg,
}
"#;

const ENUM_WITH_PREFIXES_AND_SUFFIXES: &str = r#"
enum Cake {
   DoubleCakeBlackForestGreen,
   DoubleCakeHummingbirdForestGreen,
   DoubleCakeBattenbergForestGreen,
}
"#;

#[test]
fn enum_with_suffixed_name_diagnostics() {
    test_lint_diagnostics!(ENUM_WITH_SUFFIXED_NAME, @r"
    warning: Plugin diagnostic: All enum variants are prefixed or suffixed by the same characters.
     --> lib.cairo:2:1
      |
    2 | / enum Cake {
    3 | |    BlackForestCake,
    4 | |    HummingbirdCake,
    5 | |    BattenbergCake,
    6 | | }
      | |_-
      |
    ");
}

#[test]
fn enum_with_prefixed_name_diagnostics() {
    test_lint_diagnostics!(ENUM_WITH_PREFIXED_NAME, @r"
    warning: Plugin diagnostic: All enum variants are prefixed or suffixed by the same characters.
     --> lib.cairo:2:1
      |
    2 | / enum Cake {
    3 | |    CakeBlackForest,
    4 | |    CakeHummingbird,
    5 | |    CakeBattenberg,
    6 | | }
      | |_-
      |
    ")
}

#[test]
fn enum_with_suffix_diagnostics() {
    test_lint_diagnostics!(ENUM_WITH_SUFFIX, @r"
    warning: Plugin diagnostic: All enum variants are prefixed or suffixed by the same characters.
     --> lib.cairo:2:1
      |
    2 | / enum Wood {
    3 | |     ForestBlack,
    4 | |     ForestWhite,
    5 | | }
      | |_-
      |
    ")
}

#[test]
fn enum_with_prefix_diagnostics() {
    test_lint_diagnostics!(ENUM_WITH_PREFIX, @r"
    warning: Plugin diagnostic: All enum variants are prefixed or suffixed by the same characters.
     --> lib.cairo:2:1
      |
    2 | / enum Wood {
    3 | |     BlackForest,
    4 | |     WhiteForest,
    5 | | }
      | |_-
      |
    ")
}

#[test]
fn enum_with_prefixes_and_suffixes_diagnostic() {
    test_lint_diagnostics!(ENUM_WITH_PREFIXES_AND_SUFFIXES, @r"
    warning: Plugin diagnostic: All enum variants are prefixed or suffixed by the same characters.
     --> lib.cairo:2:1
      |
    2 | / enum Cake {
    3 | |    DoubleCakeBlackForestGreen,
    4 | |    DoubleCakeHummingbirdForestGreen,
    5 | |    DoubleCakeBattenbergForestGreen,
    6 | | }
      | |_-
      |
    ")
}
#[test]
fn enum_with_not_all_prefixed_name_diagnostics() {
    test_lint_diagnostics!(ENUM_WITH_NOT_ALL_PREFIXED, @"")
}

#[test]
fn enum_with_not_all_suffixed_name_diagnostics() {
    test_lint_diagnostics!(ENUM_WITH_NOT_ALL_SUFFIXED, @r"")
}

#[test]
fn enum_with_not_camel_case_suffix() {
    test_lint_diagnostics!(ENUM_WITH_NOT_CAMEL_CASE_SUFFIX, @r"")
}

#[test]
fn enum_with_not_camel_case_prefix() {
    test_lint_diagnostics!(ENUM_WITH_NOT_CAMEL_CASE_PREFIX, @r"")
}

#[test]
fn enum_test() {
    test_lint_diagnostics!(ENUM_SINGLE, @r"");
}

#[test]
fn enum_with_suffixed_name_fixer() {
    test_lint_fixer!(ENUM_WITH_SUFFIXED_NAME, @r"
    enum Cake {
       BlackForest,
       Hummingbird,
       Battenberg,
    }
    ");
}

#[test]
fn enum_with_prefixed_name_fixer() {
    test_lint_fixer!(ENUM_WITH_PREFIXED_NAME, @r"
    enum Cake {
       BlackForest,
       Hummingbird,
       Battenberg,
    }
    ");
}

#[test]
fn enum_with_prefix_fixer() {
    test_lint_fixer!(ENUM_WITH_PREFIX, @r"
    enum Wood {
        Black,
        White,
    }
    ");
}

#[test]
fn enum_with_suffix_fixer() {
    test_lint_fixer!(ENUM_WITH_SUFFIX, @r"
    enum Wood {
        Black,
        White,
    }
    ");
}

#[test]
fn enum_with_not_all_prefixed_fixer() {
    test_lint_fixer!(ENUM_WITH_NOT_ALL_PREFIXED, @r"
    enum Cake {
       CakeBlackForest,
       CakeCakeHummingbird,
       Battenberg,
    }
    ");
}

#[test]
fn enum_with_not_all_suffixed_fixer() {
    test_lint_fixer!(ENUM_WITH_NOT_ALL_SUFFIXED, @r"
    enum Cake {
       BlackForestCake,
       HummingbirdCake,
       Battenberg,
    }
    ");
}

#[test]
fn enum_with_not_camel_case_suffix_fixer() {
    test_lint_fixer!(ENUM_WITH_NOT_CAMEL_CASE_SUFFIX, @r"
    enum Cake {
       BlackForestcake,
       Hummingbirdcake,
       Battenbergcake,
    }
    ");
}

#[test]
fn enum_single_fixer() {
    test_lint_fixer!(ENUM_SINGLE, @r"
    enum Cake {
       BlackForest,
    }
    ");
}

#[test]
fn enum_with_not_camel_case_prefix_fixer() {
    test_lint_fixer!(ENUM_WITH_NOT_CAMEL_CASE_PREFIX, @r"
    enum Cake {
       CakeblackForest,
       Cakehummingbird,
       Cakebattenberg,
    }
    ");
}

#[test]
fn enum_with_prefixes_and_suffixes_fixer() {
    test_lint_fixer!(ENUM_WITH_PREFIXES_AND_SUFFIXES, @r"
    enum Cake {
       Black,
       Hummingbird,
       Battenberg,
    }
    ");
}
