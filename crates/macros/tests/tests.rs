
#[test]
pub fn pass_expect_expanded() {
    macrotest::expand("tests/core/*.rs");
    macrotest::expand("tests/common/*.rs");
}
