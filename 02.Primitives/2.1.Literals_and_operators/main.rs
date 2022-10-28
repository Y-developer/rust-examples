fn main() {
    let num_1: u32 = 1_000_000;
    println!("one million is written as {}", num_1);

    let float_1 = 0.000_01f32;
    println!("a float {}", float_1);

    println!("");
    println!("boolean logic");
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    println!("");
    println!("Bitwise operations");
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
