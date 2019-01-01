// compile-flags: --test

#[test]
#[should_panic = "foo"]
//~^ WARN: attribute must be of the form:
//~| ERROR: attribute must be of the form `#[should_panic]` or `#[should_panic(...)]`
fn test1() {
    panic!();
}

#[test]
#[should_panic(expected)]
//~^ WARN: argument must be of the form:
fn test2() {
    panic!();
}

#[test]
#[should_panic(expect)]
//~^ WARN: argument must be of the form:
fn test3() {
    panic!();
}

#[test]
#[should_panic(expected(foo, bar))]
//~^ WARN: argument must be of the form:
fn test4() {
    panic!();
}

#[test]
#[should_panic(expected = "foo", bar)]
//~^ WARN: argument must be of the form:
fn test5() {
    panic!();
}
