pub mod basic_a;
pub mod basic_b;
pub mod basic_c;
pub mod basic_d;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_basic_a() {
        assert_eq!(::basic_a::a(), true);
    }

    #[test]
    fn test_basic_a_sub() {
        assert_eq!(::basic_a::basic_sub::sub_a(), true);
    }

    #[test]
    fn test_basic_b() {
        assert_eq!(::basic_b::a(), true);
    }

    #[test]
    fn test_basic_b_sub() {
        assert_eq!(::basic_b::basic_sub::sub_a(), true);
    }

    #[test]
    fn test_basic_c() {
        assert_eq!(::basic_c::a(), true);
    }

    #[test]
    fn test_basic_c_sub() {
        assert_eq!(::basic_c::basic_sub::sub_a(), true);
    }

    #[test]
    fn test_basic_d() {
        assert_eq!(::basic_d::a(), true);
    }

}
