# 11.1. Creating a Library

crate එකකින් library එකක් සෑදීම මෙම කොටසින් ඉගැන්වේ.
<br>

මෙහි ඇති `some_file.rs` යන crate එක library එකක් බවට පත් කිරීමට පහත command එක භාවිතා කල හැක.
```
rustc some_file.rs --crate-type=lib
```
එවිට `libsome_file.rlib` යන library එක සකස් වේ. සෑම විටම library එකකට `lib` යන prefix එක යෙදේ. එසේම library එකක file extention එක `.rlib` වේ.

එසේම තමන්ට කැමති නමකින්ද library එකක් සෑදීමේ හැකියාව ඇත. `libapple.rlib` යන library එක සෑදීමට පහත command එක භාවිතා කල හැක.
```
rustc some_file.rs --crate-type=lib --crate-name=apple
```
මෙහිදී සිදුකරන්නේ `--crate-name` යන flag එකට අදාල නම ලබා දීමයි. කෙසේ නමුත් `lib` යන prefix එක සෑම විටම library එකකට යෙදේ.