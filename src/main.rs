pub mod calculator1;

#[test]
fn calculator1() {
    assert!(calculator1::parse_Term("99").is_ok());
    assert!(calculator1::parse_Term("(42)").is_ok());
    assert!(calculator1::parse_Term("(((((11)))))").is_ok());
    assert!(calculator1::parse_Term("((665)").is_err());
}

pub mod calculator2;

#[test]
fn calculator2() {
    assert!(calculator2::parse_Term("99").is_ok());
    assert!(calculator2::parse_Term("(42)").is_ok());
    assert!(calculator2::parse_Term("(((((11)))))").is_ok());
    assert!(calculator2::parse_Term("((665)").is_err());
}

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
