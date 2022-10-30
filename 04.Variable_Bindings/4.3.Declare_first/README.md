# 4.3. Declare first

පළමුව variable එකක් declare කර පසුව එය initialize කිරීමද rust වලදී සිදුකල හැක.\
මෙසේ සිදු කිරීමෙන් initialize නොකරන ලද variables සෑදීමේ හැකියාවක් ඇති නිසා මෙය සිදුකල යුත්තේ කලාතුරකිනි.\
initialize නොකරන ලද variables සෑදුනහොත් compile කිරීමේදී errors ඇති වේ.

```rust
fn main() {
    // variable එක declare කිරීම
    let a_binding: i32;

    {
        let x = 2;
        
        // variable එක initialize කිරීම
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);
}
```