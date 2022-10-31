# 8.1. if/else

**if-else** අනෙක් programming language වලට සමාන වේ. අනෙක් programming language වල මෙන් rust වලදී boolean condition එක වරහනක් තුල දැමීමට අත්‍යවශ්‍ය නොවේ.\
එසේම සෑම if else branch එකක්ම block `{}` එකක් වේ.
```rust
let n = 5;
if n < 0 {
    print!("{} is negative", n);
} else if n > 0 {
    print!("{} is positive", n);
} else {
    print!("{} is zero", n);
}
```

if-else condition expression එකක් ලෙසද ලිවිය හැක. නමුත් මෙහිදී සෑම branch එකකම සමාන type එකක් return කල යුතුය.
```rust
let n = 5;
let big_n = if n < 10 && n > -10 {
                println!(", and is a small number, increase ten-fold");
                10 * n // return i32
            } else {
                println!(", and is a big number, halve the number");
                n / 2 // return i32
            };
```