# 2. Primitives
මෙම පාඩමේදී rust වල පාවිච්චි කල හැකි primitives පිළිබදව ඉගැන්වේ.

## Scalar Types
- Signed integers: `i8`, `i16`, `i32`, `i64`, `i128` and `isize`
- Unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128` and `usize`
- Floating point: `f32` and `f64`
- Character: `char`
- Booleans: `bool`
- Unit type: `()` මෙහි තැබිය හැකි එකම අගය empty tuple එකක් වේ. මෙහි අගයන් කිහිපයක් ඇතුලත් කිරීමට නොහැකි නිසා මෙය compound type එකක් ලෙස සැලකිය නොහැක.

## Compound Types
- arrays like [1, 2, 3]
- tuples like (1, true)

හැකි සෑම විටම variable එකක type එක සදහන් කල යුතුය.
```rust
let logical: bool = true;
let a_float: f64 = 29.362;
let love_face: char = '\u{1F970}';
```

numbers වල නම් එහි අගින්ද (suffix එකක් ලෙස) type එක සදහන් කල හැක. suffix type වලංගු වන්නේ පහත type සදහා පමණි.\
`u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `u128`, `i128`, `usize`, `isize`,	`f32`, `f64`
```rust
let count_of_mangos = 5i32;
let pi = 3.1415f32;
```

number එකක් ආකාරයේ variable එකක type එකක් සදහන් කර නැත්නම් float එකක් `f64` ලෙසද integer එකක් `i32` එකක් ලෙසද ගැනේ.
```rust
let num_1 = 58;
let float_1 = 36.58;
```

number එකක් ආකාරයේ variable එකක type එක පසුවද සදහන් කල හැක.
```rust
let mut inferred_type = 12;
inferred_type = 4294967296i64;
```

mutable variable එකකට පමණක් පසුව අගයක් ආදේශ කල හැක.
```rust
let mut num_2:  i32 = 12; // Mutable `i32`
mutable = 21;
```

variable එකක type එක පසුව වෙනස් කල නොහැක.
```rust
let mut num_3:  i8 = 12;
num_3 = ture // show Error
```

නමුත් එකම නමින් variable සෑදීමේ හැකියාව ඇත. මෙය shadowing ලෙස හදුන්වයි. නමුත් access කල හැක්කේ අවසානයේදී සාදන ලද variable එකයි. අනිත් variable එක access කිරීමට හැකියාවක් නැතිවී යයි.
```rust
let val_x: i32 = 678;
let val_x: i32 = 123;
println!("{}", val_x);
// 123
```