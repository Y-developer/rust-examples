# 8.2.1. Nesting and labels

loop එකක් තුල තවත් loop එකක් යෙදීමේදී එම loop නම් කිරීමට `'label` යෙදීමේ හැකියාව ඇත.\
`break` හෝ `continue` statements යෙදීමේදි මෙම `'label` භාවිතා කිරීමේ හැකියාව ඇත.
```rust
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
```