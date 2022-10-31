# 8.2.2. Returning from loops

loop එකක break අවස්ථාවදී value එකක් return කිරීමේ හැකියාව ඇත. මේ සදහා `break` පසුව අදාල return කරන value එක යෙදිය යුතුය.

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; //මෙම අවස්ථාවේදී value එක return වේ.
        }
    };

    println!("result = {}", result);
}
```