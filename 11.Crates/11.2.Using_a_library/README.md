# 11.2. Using a Library

`my_exe.rs` යන crate එකට 11.1 දී සාදාගත් `libsome_file.rlib` library එක සම්බන්ධ කිරීම පහත ආකාරයට සිදුකල හැක.මෙහිදී භාවිතා කරන්නේ `--extern` යන flag එකයි.

```
rustc my_exe.rs --extern my_lib=../11.1.Creating_a_library/libsome_file.rlib --edition=2018
```
මෙහිදී `libsome_file.rlib` library එක `my_lib` යන module එක යටතේ import කෙරේ. මෙය සාමාන්‍ය module එකක් මෙන් `my_exe.rs` crate එක තුලදී භාවිතා කල හැක.