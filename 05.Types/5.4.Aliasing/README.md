# 5.4. Aliasing (අනුවර්ථ නාමයක් ලබා දීමෙන්)

primitive type වලට වෙනත් නමක් ලබාදීම මෙහිදී සිදු කරයි. සාමාන්‍යයෙන් **අනුවර්ථ නාමය UpperCamelCase** විය යුතුය.

```rust
// `NanoSecond`, `Inch`, සහ `U64`, u64 සදහා අනුවර්ථ නාමයන් වේ.
type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    // `NanoSecond` = `Inch` = `U64` = `u64`.
    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;

    // Note that type aliases *don't* provide any extra type safety, because
    // aliases are *not* new types
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}

```