// Bind the `deeply::nested::function` path to `other_function`.
use deeply::nested::{
    function_one as my_function_one,
    function_two as my_function_two
};

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function_one() {
            println!("called `deeply::nested::function_one()`");
        }

        pub fn function_two() {
            println!("called `deeply::nested::function_two()`");
        }

        pub fn function_three() {
            println!("called `deeply::nested::function_three()`");
        }
    }
}

fn main() {
    // Easier access to `deeply::nested::function`
    my_function_one();
    my_function_two();

    println!("Entering block");
    {
        // This is equivalent to `use deeply::nested::function as function`.
        // This `function()` will shadow the outer one.
        use crate::deeply::nested::function_three;

        // `use` bindings have a local scope. In this case, the
        // shadowing of `function()` is only in this block.
        function_three();

        println!("Leaving block");
    }

    function();
}