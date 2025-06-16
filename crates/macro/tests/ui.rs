use trybuild::TestCases;

#[test]
fn ui_tests() {
    let t = TestCases::new();
    t.pass("tests/ui/01-domain.rs");
    t.pass("tests/ui/02-domain-impl.rs");
}
