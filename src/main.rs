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

pub mod calculator2b;

#[test]
fn calculator2b() {
    assert_eq!(calculator2b::parse_Term("11").unwrap(), "11");
    assert_eq!(calculator2b::parse_Term("foo42").unwrap(), "id(foo42)");
    assert_eq!(calculator2b::parse_Term("((22))").unwrap(), "twenty-two");
    assert_eq!(calculator2b::parse_Term("(222)").unwrap(), "222");
}

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
