# 4.4. Freezing

අගය වෙනස් කල හැකි(mutable) variable එකක් එම නමින්ම ඇති අගය වෙනස් කල නොහැකි(immutable) variable එකකට සමාන කිරීම freezing කිරීම ලෙස හදුන්වයි.\
වෙනත් ආකාරයකට කිවහොත් mutable variable එකක් immutable variable එකක් බවට shadowing කිරීම freezing කිරීම ලෙස හදුන්වයි.\
immutable variable පවතින scope එකෙන් ඉවත් වන තෙක් frozen variable එකක value එකක් වෙනස් කල නොහැක.

```rust
fn main() {
    // mutable variable එකක් සෑදීම
    let mut num_1 = 7i32;

    {
        // block එක තුලදී
        // num_1 variable එක immutable variable එක්ක බවට shadow කිරීම
        let num_1 = num_1;

        println!("frozen num_1 = {}", num_1);

        // දැන් මෙම block එක තුලදී num_1 ට අගයක් ආදේශ කල නොහැක.
        // එනම් block එක තුලදී num_1 immutable වී ඇත.
        // මෙයට num_1 frozen වී ඇතැයි කියනු ලැබේ.
    }

    // නමුත් block එකට පිටින් සිට num_1 ට අගයක් ආදේශ කල හැක.
    // මීට හේතුව immutable variable එක සීමා වන්නේ block එක තුලට පමණි.
    num_1 = 3;

    println!("num_1 = {}", num_1);
}
```