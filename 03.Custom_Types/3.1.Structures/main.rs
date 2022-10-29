struct Person {
    name: String,
    age: u8,
    is_marray: bool,
    last_name_start_letter: char,
}

struct Card(i8, char, bool);

fn main() {
    // අවුරුදු 25 ක් වයස වන විවාහ වී නොමැති Kasun Sirisena ගේ object එක
    let kasun = Person {
        name: String::from("Kasun"),
        age: 25,
        is_marray: false,
        last_name_start_letter: 'S',
    };
    println!("");
    println!("++++++++++ Make an object ++++++++++");
    println!("name: {} {}.", kasun.name, kasun.last_name_start_letter);
    println!("age: {}", kasun.age);
    println!("is married: {}", kasun.is_marray);

    // අවුරුදු 25 ක් වයස වන විවාහ වී නොමැති Nimal Disanayake ගේ object එක
    let nimal = Person {
        name: String::from("Nimal"),
        last_name_start_letter: 'D',
        ..kasun
    };
    println!("");
    println!("++++++++++ With same field ++++++++++");
    println!("name: {} {}.", nimal.name, nimal.last_name_start_letter);
    println!("age: {}", nimal.age);
    println!("is married: {}", nimal.is_marray);

    let Person {
        name: _name,
        age,
        is_marray,
        last_name_start_letter: last_letter,
    } = nimal;
    println!("");
    println!("++++++++++ Destructure ++++++++++");
    println!("name: {} {}.", _name, last_letter);
    println!("age: {}", age);
    println!("is married: {}", is_marray);

    let card_1 = Card(10, 'K', false);
    println!("");
    println!("++++++++++ Tuple structure ++++++++++");
    println!("card_1 : {}, {}, {}", card_1.0, card_1.1, card_1.2);

    println!("");
    println!("++++++++++ Destructure tupel sturct ++++++++++");
    let Card(num, character, boolean) = card_1;
    println!("card_1 : {}, {}, {}", num, character, boolean);
}
