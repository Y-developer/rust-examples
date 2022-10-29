# 3.1. Structures
rust වල ආකාර තුනක struture සෑදීමේ හැකියාව ඇත.
- Classic C like structs
- Tuple structs
- Unit structs
  
## Classic C like struct
```rust
struct Person{
    name: String,
    age: u8,
}
```

## Tuple structs
```rust
struct Pair(i32, f32);
```

## Unit structs
```rust
struct Unit;
```

## stucture වල භාවිත
එක් sturcture එකක් තවත් structure එකක් සෑදීම සඳහා භාවිතා කල හැක.
```rust
struct Point{
    x: i32,
    y: i32,
}

struct Rectangle{
    top_left: Point,
    bottom_left: Point,
    top_right: Point,
    bottom_right: Point,
}
```

යම් structure එකක් භාවිතයෙන් object එකක් පහත පරිදි සෑදිය හැක.
```rust
struct Person{
    name: String,
    age: u8,
    is_marray: bool,
    last_name_start_letter: char,
}

// අවුරුදු 25 ක් වයස වන විවාහ වී නොමැති Kasun Sirisena ගේ object එක 
let kasun = Person{
    name: String::from("Kasun"),
    age: 25,
    is_marray: false,
    last_name_start_letter: 'S'
};
```

struct එක භාවිතයෙන් සාදන ලද object එකේ fields කියවීම පහත පරිදි සිදුකල හැක.
```rust
println!("name: {} {}.", kasun.name, kasun.last_name_start_letter);
println!("age: {}", kasun.age);
println!("is married: {}", kasun.is_marray);
```

අලුතෙන් object එකක් සෑදීමේදී එහි සමහර fields කලින් සාදන ලද object එකකට සමාන නම් එය පහත පරිදි සිදුකල හැක.
```rust
// අවුරුදු 25 ක් වයස වන විවාහ වී නොමැති Nimal Disanayake ගේ object එක
let nimal = Person {
    name: String::from("Nimal"),
    last_name_start_letter: 'D',
    ..kasun
}
```

struct එක භාවිතයෙන් සාදන ලද object එකක් destructure කිරීම පහත පරිදි සිදුකල හැක.
```rust
let Person {
    name: _name,
    age,
    is_marray,
    last_name_start_letter: last_letter,
} = nimal;

println!("name: {} {}.", _name, last_letter);
println!("age: {}", age);
println!("is married: {}", is_marray);
```

tuple structure එක්ක භාවිතයෙන් object එකක් සෑදීම පහත පරිදි සිදුකල හැක.
```rust
struct Card (i8, char, bool);
let card_1 = Card(10, 'K', false);
```

tuple object එකකින් elements ලබා ගැනීම පහත පරිදි සිදුකල හැක.
```rust
println!("card_1 : {}, {}, {}", card_1.0, card_1.1, card_1.2);
```

tuple object එක්ක destructure කිරීම පහත පරිදි සිදුකල හැක.
```rust
let Card(num, character, boolean) = card_1;
println!("card_1 : {}, {}, {}", num, character, boolean);
```

unit structure එකක් භාවිතයෙන් object එකක් සෑදීම.
```rust
struct Unit;
let _unit = Unit;
```
