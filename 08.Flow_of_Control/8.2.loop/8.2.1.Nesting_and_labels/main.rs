#![allow(unreachable_code)]
#![allow(unused_labels)]

fn main() {

    // loop_1 යනු පිටතින් පිහිටි loop එකයි.
    // loop_2 යනු ඇතුලතින් පිහිටි loop එකයි.

    'loop_1: loop {
        println!("Entered the outer loop");

        'loop_2: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'loop_1;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}