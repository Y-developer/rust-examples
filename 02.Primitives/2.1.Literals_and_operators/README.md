# 2.1. Literals and operators

numaric variable කියවීම පහසුකර ගැනීම සදහා underscore භාවිතා කර හැක.
```rust
let num_1:u32 = 1_000_000;
println!("one million is written as {}", num_1);
//one million is written as 1000000

let float_1 = 0.000_01f32;
println!("a float {}", float_1);
//a float 0.00001
```

ඉදිරිපිට prefix එකක් එකතු කිරීමෙන් විවිද පාදයෙන් යුත් සංඛ්‍යා ලිවිය හැක.
|පාදය|prefix|
|-|-|
|hexadecimal|`0x`|
|octal|`0o`|
|binary|`0b`|
```rust
let hex_1: u32 = 0x2B1; // hexadecimal 689
let octal_1: u32 = 1261; // hexadecimal 689
let bin_1: u32 = 1010110001; // binary number 689
```

## boolean logic
```rust
println!("true AND false is {}", true && false);
println!("true OR false is {}", true || false);
println!("NOT true is {}", !true);
```

## Bitwise operations
```rust
println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
println!("1 << 5 is {}", 1u32 << 5);
println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
```