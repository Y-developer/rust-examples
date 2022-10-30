fn main() {
    // mutable variable එකක් සෑදීම
    let mut num_1 = 7i32;

    {
        // block එක තුලදී
        // num_1 variable එක immutable variable එක්ක බවට shadow කිරීම
        let num_1 = num_1;

        println!("frozon num_1 = {}", num_1);

        // දැන් මෙම block එක තුලදී num_1 ට අගයක් ආදේශ කල නොහැක.
        // එනම් block එක තුලදී num_1 immutable වී ඇත.
        // මෙයට num_1 frozon වී ඇතැයි කියනු ලැබේ.
    }

    // නමුත් block එකට පිටින් සිට num_1 ට අගයක් ආදේශ කල හැක.
    // මීට හේතුව immutable variable එක සීමා වන්නේ block එක තුලට පමණි.
    num_1 = 3;

    println!("num_1 = {}", num_1);
}
