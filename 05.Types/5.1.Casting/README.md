# 5.1. Casting

`as` keyword එක භාවිතයෙන් එක් primitive type එකක් තවත් primitive type එකක් බවට පරිවර්ථනය කල හැක. නමුත් සිදු කල හැක්කේ ප්‍රශ්ණයක් මතු නොවන ආකාරයේ පරිවර්ථන පමණි. C වල පරිවර්ථන සම්මුතීන් rust වලදීද අනුගමනය කරයි.

`f32` වර්ගයේ float එකක් `u8` බවට පරිවර්ථනය කිරීම.
```rust
let float_1 = 65.4321_f32;
let num_1: u8 = float_1 as u8;
println!("float_1 = {}", num_1);
```

`u8` වර්ගයේ integer එකක් `char` එකක් බවට පරිවර්ථනය.
මෙසේ char එකක් බවට පරිවර්ථනය කල හැක්කේ u8 වර්ගයේ integers පමණි.
```rust
let letter: char = num_1 as char;
println!("letter = {}", letter);
```

එක් integer type එකක් තවත් interger type එකක් බවට පරිවර්ථනය කිරීමේදී එයට එම අගය තබා ගැනීමට හැකියාව තිබිය යුතුය. නැතහොත් compile කිරීමේදී errors ඇතිවේ.
```rust
let num_a: u32 = 1000;
let num_b: u8 = num_a as u8;
// 1000 u8 හි උපරිම අගයට වඩා වැඩි නිසා මෙය සිදුකල නොහැක.
```

float එක්ක integer එකක් බවට පරිවර්ථනය කිරීමේදී,
- float එක integer type එකේ උපරිමයට වඩා වැඩිනම් එම integer type එකේ උපරිම අගය ලැබේ.
```rust
let float_2: f32 = 375.6593;
let num_2: i8 = float_2 as i8;
println!("num_2 = {}", num_2);
// num_2 = 127
// i8 වල උපරිම අගය 127 වේ.
```
- float එක integer type එකේ අවම අගයට වඩා අඩුනම් එම integer type එකේ උපරිම අවම අගය ලැබේ.
```rust
let float_3: f32 = -200.5648;
let num_3: i8 = float_3 as i8;
println!("num_3 = {}", num_3);
// num_2 = -128
// i8 වල අවම අගය -128 වේ.
```

