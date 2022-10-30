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