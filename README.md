# Rust Programming Language

## Install Rust
https://www.rust-lang.org/tools/install \
\
නිවැරිව Rust install වී ඇද්දැයි බැලීම \
- rust version එක සෙවීම `rustup --version`
- rust compiler එකේ version එක සෙවීම `rustc --version`
- rust package manager එකේ version එක සෙවීම `cargo --version`

## Table of Content
1. [Hello World]<!--(01.Hello_World/README.md)-->
2.  [Primitives](02.Primitives/README.md)
    1.  [Literals and operators](02.Primitives/2.1.Literals_and_operators/README.md)
    2.  [Tuples](02.Primitives/2.2.Tuples/README.md)
    3.  [Arrays and Slices](02.Primitives/2.3.Arrays_and_Slices/README.md)
3.  [Custom Types](03.Custom_Types/README.md)
    1.  [Structures](03.Custom_Types/3.1.Structures/README.md)
    2.  [Enums](03.Custom_Types/3.2.Enums/README.md)
        1.  [use](03.Custom_Types/3.2.Enums/3.2.1.use/README.md)
        2.  [C-like](03.Custom_Types/3.2.Enums/3.2.2.C-like/README.md)
        3.  [Testcase: linked-list]<!--(03.Custom_Types/3.2.Enums/3.2.3.Testcase_linked-list/README.md)-->
    3. [constants]<!--(03.Custom_Types/3.3.constants/README.md)-->
4.  [Variable Bindings](04.Variable_Bindings/README.md)
    1.  [Mutability](04.Variable_Bindings/4.1.Mutability/README.md)
    2.  [Scope and Shadowing](04.Variable_Bindings/4.2.Scope_and_Shadowing/README.md)
    3.  [Declare first]
    4.  [Freezing]
5.  [Types]<!--(05.Types/README.md)-->
6.  [Conversion]<!--(06.Conversion/README.md)-->
7.  [Expressions]<!--(07.Expressions/README.md)-->
8.  [Flow of Control]<!--(08.Flow_of_Control/README.md)-->
9.  [Functions]<!--(09.Functions/README.md)-->
10. [Modules](10.Modules/README.md)
    1.  [Visibility](10.Modules/10.1.visibility/README.md)
    2.  [Struct visibility](10.Modules/10.2.struct_visibility/README.md)
    3.  [The use declaration](10.Modules/10.3.the_use_declaration/README.md)
    4.  [super and self](10.Modules/10.4.super_and_self/README.md)
    5.  [File hierarchy](10.Modules/10.5.file_hierarchy/README.md)
11. [Crates](11.Crates/README.md)
    1.  [Creating a Library](11.Crates/11.1.Creating_a_library/README.md)
    2.  [Using a Library](11.Crates/11.2.Using_a_library/README.md)
12. [Cargo]<!--(12.Cargo/README.md)-->
13. [Attributes]<!--(13.Attributes/README.md)-->
14. [Generics]<!--(14.Generics/README.md)-->
15. [Scoping rules]<!--(15.Scoping_rules/README.md)-->
16. [Traits]<!--(16.Traits/README.md)-->
17. [macro_rules!]<!--(17.macro_rules/README.md)-->
18. [Error handling]<!--(18.Error_handlling/README.md)-->
19. [Std library types]<!--(19.Std_library_types/README.md)-->
20. [Std misc]<!--(20.Std_misc/README.md)-->
21. [Testing]<!--(21.Testing/README.md)-->
22. [Unsafe Operations]<!--(22.Unsafe_Operations/README.md)-->
23. [Compatibility]<!--(23.Compatibility/README.md)-->
24. [Meta]<!--(24.Meta/README.md)-->
## වැදගත් Commands
| Commands                      | Description                   |
| ----------------------------- | ----------------------------- |
| `rustc <file_name>`           | .rs file එකක් compile කිරීම      |
| `<file_name>`                 | compile වූ project එක run කිරීම  |
| `cargo init`                  | අලුත් rust project එකක් ආරම්භ කිරීම |
| `cargo build`                 | project එක build කිරීම          |
| `target/debug/<project_name>` | build වූ project එක run කිරීම    |
| `cargo run`                   | project එක build කරමින් run කිරීම |


