# 8.3. while
වෙනත් programming language වල මෙන්ම මෙහිදීද `while` loop එක භාවිතා කල හැක. condition එක `true` වන තෙක් loop එක තුල ඇති statement run වේ.
```rust
fn main() {
    // A counter variable
    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }
}
```