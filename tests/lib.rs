extern crate mylib;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_basic_a() {
    assert_eq!(mylib::basic_a::a(), true);
}

#[test]
fn test_basic_a_sub() {
    assert_eq!(mylib::basic_a::basic_sub::sub_a(), true);
}

#[test]
fn test_basic_b() {
    assert_eq!(mylib::basic_b::a(), true);
}

#[test]
fn test_basic_b_sub() {
    assert_eq!(mylib::basic_b::basic_sub::sub_a(), true);
}

#[test]
fn test_basic_c() {
    assert_eq!(mylib::basic_c::a(), true);
}

#[test]
fn test_basic_c_sub() {
    assert_eq!(mylib::basic_c::basic_sub::sub_a(), true);
}

#[test]
fn test_basic_d() {
    assert_eq!(mylib::basic_d::a(), true);
}
