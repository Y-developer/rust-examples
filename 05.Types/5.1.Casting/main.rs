fn main() {
    // f32 to u8
    let float_1 = 65.4321_f32;
    let num_1: u8 = float_1 as u8;
    println!("float_1 = {}", num_1);

    // u8 to char
    let letter: char = num_1 as char;
    println!("letter = {}", letter);

    let float_2: f32 = 375.6593;
    let num_2: i8 = float_2 as i8;
    println!("num_2 = {}", num_2);

    let float_3: f32 = -200.5648;
    let num_3: i8 = float_3 as i8;
    println!("num_3 = {}", num_3);
}
