# 4.2. Scope and Shadowing

## Variable Scope
සෑම variable එකකටම scope එකක් ඇති අතර එහි සජීවි බව යම් block එකකට පමණක් සීමා වේ.\
block එකක් යනු සගල වරහන් දෙකකින් `{ }` වෙන් කර ඇති ප්‍රදේශයකි.
```rust
fn main() {
    // num_1 ජීවත් වන්නේ main function scope එක තුලයි.
    let num_1: i32 = 1;

    // මෙය main function එක තුල වූ කුඩා block එකකි.
    {
        // num_2 ජීවත් වන්නේ මෙම කුඩා block එක තුල පමණි.
        let num_2: i32 = 2;

        // මෙ කුඩා block එක තුල සිට num_1 හා num_2 යන දෙකම access කල හැක.
        println!("inside block | num_1 =  {}", num_1);
        println!("inside block | num_2 =  {}", num_2);
    }

    // කුඩා block එකට පිටින් සිට num_2 access කල නොහැක.
    // නමුත් මෙහි සිට num_1 access කල හැක.
    println!("outside block | num_1 =  {}", num_1);
    // println!("outside block | num_2 =  {}", num_2);
    // ^ comment එක ඉවත් කර බලන්න.
}
```

## Variable Shadowing

ඉහත ආකාරයටම විවිධ scope තුල සිට එකම නමකින් variable සෑදීමේ හැකියාව (shadowing) ඇත.
```rust
fn main() {
    let num_1: i32 = 50;

    // මෙය main function එක තුල වූ කුඩා block එකකි.
    {   
        // shadow කිරීමට පෙර
        println!("inside block befor shadowed: num_1 = {}", num_1);

        // shadow කිරීම
        let num_1: i32 = 100;

        // shadow කල පසු
        println!("inside block after shadowed: num_1 = {}", num_1);
    }

    // shadow කිරීම block එක තුල සිදුවූ නිසා
    // එහි බලපෑම block එකෙන් පිටතට සිදුනොවේ.
    println!("outside block after shadowed: num_1 = {}", num_1);
}
```