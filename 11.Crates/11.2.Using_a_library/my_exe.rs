fn main() {
    my_lib::public_function();

    // Error! `private_function` is private
    //rary::private_function();

    my_lib::indirect_access();
}