# 8.2. loop
අනන්තය දක්වා run වීම සදහා run වීම සදහා rust වල `loop` keyword එක භාවිතා කරයි.\
`brake` statement එක මගින් loop එකෙන් ඕනෑම අවස්ථාවක ඉවත් විය හැක.\
`continue` statement එක මගින් ඊට පහලින් ඇති statement run වීම වලක්වා නැවත මුල සිට loop එක්ක ආරම්භ කරවයි.
```rust
fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
}

```