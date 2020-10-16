#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-basic_atom_reaction.rs");
    t.pass("tests/reversible_atom");
    //t.pass("tests/02-parse-body.rs");
    //t.compile_fail("tests/03-expand-four-errors.rs");
    //t.pass("tests/04-paste-ident.rs");
    //t.pass("tests/05-repeat-section.rs");
    //t.pass("tests/06-make-work-in-function.rs");
    //t.pass("tests/07-init-array.rs");
    //t.pass("tests/08-inclusive-range.rs");
    //t.compile_fail("tests/09-ident-span.rs");
    //t.pass("tests/10-interaction-with-macrorules.rs");
}
