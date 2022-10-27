// module එකක් සාදන ආකරය.
// මෙම module එකේ නම my_mod වේ.
mod my_mod {
    // මෙය private අවස්ථාවේ පවතින item එකකි.
    // එම නිසා module එකට පිට සිට access කල නොහැක.
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // මෙය public අවස්ථාවේ පවතින item එකකි.
    // එම නිසා module එකට පිටින් සිට access කල හැක.
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // private වුවත් public වුවත් module එක තුල සිට ඹ්නෑම item එකකට access කල හැක.
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // module එකක් තුල තවත් module එකක් සෑදීමේ හැකියාව ඇත.
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // `pub(in path)` syntax එක භාවිතා කල විට ලබා දෙන path එකට පමණක් මෙය visible වේ.
        // ලබා දෙන path එක අනිවාර්යයෙන්ම parent එකක් හෝ ඉහලින් පිහිටි parent කෙනෙක් විය යුතුය.
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }

        // `pub(self)` syntax එක භාවිතා කල විට current module එකකට පමණක් මෙය visible වේ.
        // මෙය private visibility එකට සමානය.
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        // `pub(super)` syntax එක භාවිතා කල විට parent module එකට පමණක් මෙය visible වේ.
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // `pub(super)` syntax එක භාවිතා කල විට current crate එකට visible වේ.
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    // මෙය private module එකකකි.
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }

        // මෙය pub(create) වුවත් private module එකක් තුල ඇති නිසා ඊට පිටින් සිට call කල නොහැක.
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

// module එකේ ඇති functions වලට call කිරීම main function එකේ සිට සිදු කරයි.
fn main() {
    // Modules allow disambiguation between items that have the same name.
    function();
    my_mod::function();

    // Public items, including those inside nested modules, can be
    // accessed from outside the parent module.
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) items can be called from anywhere in the same crate
    my_mod::public_function_in_crate();

    // pub(in path) items can only be called from within the module specified
    // Error! function `public_function_in_my_mod` is private
    //my_mod::nested::public_function_in_my_mod();
    // TODO ^ Try uncommenting this line

    // Private items of a module cannot be directly accessed, even if
    // nested in a public module:

    // Error! `private_function` is private
    //my_mod::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_function` is private
    //my_mod::nested::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    //my_mod::private_nested::function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    //my_mod::private_nested::restricted_function();
    // TODO ^ Try uncommenting this line
}