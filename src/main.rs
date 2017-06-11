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

pub mod calculator3;

macro_rules! test3 {
    ($expr:expr) => {
        println!("parsing {}", stringify!($expr));
        assert_eq!(calculator3::parse_Expr(stringify!($expr)).unwrap(), $expr);
    }
}

pub mod calculator5;

#[test]
fn calculator5() {
    assert_eq!(&format!("{:?}", calculator5::parse_Exprs("").unwrap()), "[]");
    assert_eq!(&format!("{:?}", calculator5::parse_Exprs("3 + 3 * 3").unwrap()), "[(3 + (3 * 3))]");
    assert_eq!(&format!("{:?}", calculator5::parse_Exprs("3 + 3 * 3,").unwrap()), "[(3 + (3 * 3))]");
    assert_eq!(&format!("{:?}", calculator5::parse_Exprs("3 + 3 * 3, 2 + 2").unwrap()), "[(3 + (3 * 3)), (2 + 2)]");
    assert_eq!(&format!("{:?}", calculator5::parse_Exprs("3 + 3 * 3, 2 + 2,").unwrap()), "[(3 + (3 * 3)), (2 + 2)]");
}

#[test]
fn calculator3() {
    test3!(1 + 1);
    test3!(2 + 10 + 5);
    test3!(2 + 3 * 2);
    test3!(5 + 3 * (2 + 2));
    test3!(5 + 8 / 4 + 4 / (3 - 1));
}

pub mod calculator4;
pub mod ast;

#[test]
fn calculator4() {
    assert_eq!(
        &format!("{:?}", calculator4::parse_Expr("11 * 12 + 13").unwrap()),
        "((11 * 12) + 13)"
    );
}

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
