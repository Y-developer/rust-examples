pub fn type_of<T>(_: &T) -> &str {
    return std::any::type_name::<T>();
}

fn main() {
    let logical: bool = true;
    let a_float: f64 = 29.362;
    let love_face: char = '\u{1F970}';
    println!("{} {} {}", logical, a_float, love_face);

    let count_of_mangos = 5i32;
    let pi = 3.1415f32;
    println!("{} {}", count_of_mangos, pi);

    let num_1 = 58;
    let float_1 = 36.58;
    println!("type of num_1 is {}", type_of(&num_1));
    println!("type of float_1 is {}", type_of(&float_1));

    let mut inferred_type = 12;
    println!("Befor type of inferred_type is {}", type_of(&inferred_type));
    inferred_type = 4294967296i64;
    println!("After type of inferred_type is {}", type_of(&inferred_type));

    let val_x: i32 = 678;
    let val_x: i32 = 123;
    println!("{}", val_x);
}
