# 2.2. Tuples

**විවිධ type වලින් සමන්විත variables වල collection එකක් tuple එකක් ලෙස හැදින්වේ.**
```rust
let long_tuple = (1u8, 2u16, 3u32, 4u64, -4i64, 0.1f32, 0.2f64, 'a', true);
```

index එක භාවිතයෙන් tuple එකේ අවශ්‍ය දත්ත ලබාගත හැක.
```rust
println!("long tuple first value: {}", long_tuple.5);
// long tuple first value: 0.1
```

tuple එකක සම්පූර්ණයෙන් print කරගැනීම පහත ආකාරයට සිදුකල හැක.  නමුත් print කල හැක්කේ elements 12 හෝ ඊට වඩා අඩු tuple පමණි.
```rust
println!("long_tuple is: {:?}", long_tuple);
```

tuple එකක් තුල තවත් tuple ඇතුලත් කල හැක. එයද ඉහත පරිදි print කල ගැනීමේ හැකියාව ඇත.
```rust
let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
```

එක් element එකක් ඇති tuple එකක් දැක්වීමේදී අනිවාර්යයෙන්ම එම element එකේ අගයට `,` යෙදිය යුතුය. නැතහෝත් එය සාමාන්‍ය [Scalar Types](../README.md#scalar-types) එකක් ලෙස ගැනේ.
```rust
println!("one element tuple: {:?}", (5u32,));
println!("just an integer: {:?}", (5u32));
```

පහත ආකාරයට tuple එකක් destructured කල හැක.
```rust
let tuple = (1, "hello", 4.5, true);
let (a, b, c, d) = tuple;
println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
```

tuple funtion එකක arguments වලට මෙන්ම return එක ලෙසද භාවිතා කල හැක.
```rust
// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}
```