fn main() {
    println!("print tuple & get tuple element");
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -4i64, 0.1f32, 0.2f64, 'a', true);
    println!("long tuple first value: {}", long_tuple.5);
    println!("long_tuple is: {:?}", long_tuple);

    println!("");
    println!("tuple in tuple");
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    println!("");
    println!("single element tuple");
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    println!("");
    println!("tuple destructure");
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
}
