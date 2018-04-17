extern crate mylib;

fn main() {
    
    println!("hello mylib!");

    println!("basic_a {} / sub - {}", mylib::basic_a::a(), mylib::basic_a::basic_sub::sub_a() );
    println!("basic_b {} / sub - {}", mylib::basic_b::a(), mylib::basic_b::basic_sub::sub_a() );
    println!("basic_c {} / sub - {}", mylib::basic_c::a(), mylib::basic_c::basic_sub::sub_a() );
    println!("basic_d {}", mylib::basic_a::a());

}