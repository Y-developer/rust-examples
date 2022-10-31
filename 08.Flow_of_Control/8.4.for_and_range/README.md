# 8.4. for and range

## for and range
rust වලදී `for` සහ `in` keyword භාවිතා කර for loop එක සෑදීමේ හැකියාව ඇත.\
පහසුම ආකාරය වන්නේ range notation එක භාවිතා කර for loop එක ලිවීමයි.
<br><br>

### a..b ආකාරය
`a..b` භාවිතා කල විට a සිට b (b ඇතුලත් නොවේ) දක්වා for loop එක iterate වේ.

```rust
fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
```

### a..=b ආකාරය
තවත් ක්‍රමයක් වන්නේ `a..=b` භාවිතා කිරීමයි. එවිට a සිට b (b ඇතුලත්ව) දක්වා for loop එක iterate වේ.
```rust
fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
```

## for and iterators
`iter`, `into_iter` and `iter_mut` මගින් collection විවිධ ආකාරයට iterate කිරීම සිදුකල හැකිය. 

### iter
`iter` මගින් collection එකක පවතින සෑම element එකටම අදාලව itarate වේ. නමුත් collection එකට කිසිදු වෙනසක් සිදු නොවේ.
```rust
fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    
    println!("names: {:?}", names);
}
```

### into_iter
`into_iter` මගින් collection එකක පවතින සෑම element එකටම අදාලව itarate වේ. නමුත් itarate වීම අවසානයේදී collection එක නැතිවී යයි.
```rust
fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    
    // println!("names: {:?}", names);
    // FIXME ^ Comment out this line
}
```

### iter_mut
`iter_mut` මගින් collection එකක පවතින සෑම element එකටම අදාලව itarate වේ. මෙමගින් collection එකේ element වෙනස් කල හැක.
```rust
fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}
```
