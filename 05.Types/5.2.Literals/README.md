# 5.2. Literals

සංඛ්‍යාමය literal සකස් කිරීමේදී එහි type එක අගටද යෙදිය හැක.\
සංඛ්‍යාමය literal එකක type එක නොයොදන්නේ නම්,
- integer `i32` ලෙසද,
- float `f64` ලෙසද, compiler එක සලකනු ලබයි.

```rust
fn main() {
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}
```