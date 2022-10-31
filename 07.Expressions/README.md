# 7. Expressions

බොහෝවිට rust programm එක්ක සෑදෙන්නේ statement series එකකිනි.
```rust
fn main(){
    // statement
    // statement
    // statement
}
```
rust වල statement වර්ග කීපයක් දැකගත හැක. බොහෝ අවස්තාවල දැකගත හැක්කේ ප්‍රධාන statement දෙකකි.
- variable එකක් declar කිරීම
- `;` භාවිතා කර expression එකක් ලිවීමයි.
```rust
fn main() {
    // variable binding
    let x = 5;

    // expression;
    x;
    x + 1;
    15;
}
```
block එකක් යනු expression එකකි. එය යම් variable එකකට value එකක් assign කිරීම සදහා භාවිතා කල හැක. මෙහිදී block එකේ අවසාන expression එකේ value එක variable එකට assign වේ. නමුත් අවසාන expression එක `;` කින් අවසන් වේ නම් variable එකට assign වන්නේ `()` කි.
```rust
fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
```