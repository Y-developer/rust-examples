# 4. Variable Bindings

Variable එකක type එක එය declar කරන අවස්ථාවේදීම සිදු කිරීම සාමාන්‍යයෙන් rust වලදී සිදුකරයි.
```rust
let letter: char = 't';
```

බොහෝ අවස්ථාවලදී compiler එකට variable එකක type එක එහි අන්තර්ගතය සලකා අනුමාන කල හැක.
```rust
let is_eat = true;
// is_eat bool එකක් ලෙස compiler එකට අනුමාන කල හැක.
```
පාවිච්වි නොකරන ලද variable එකක් සහිත code එකක් compile කිරීමේදී සාමාන්‍යයෙන් `warning: unused variable` යනුවෙන් warning එක්ක ලැබේ.
variable එකේ ඉදිරියට underscore `_` එක්ක යෙදීමෙන් මෙය වලක්වා ගත හැක.
```rust
let num_1: i32 = 25; // warning: unused variable පෙන්වයි.  
let _num_2: i32 = 35; // warning එකක් නොපෙන්වයි.
```