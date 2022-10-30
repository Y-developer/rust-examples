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