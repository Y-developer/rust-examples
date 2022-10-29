# 2.3. Arrays and Slices

## Arrays

array එකක් යනු එකම type එකක data එකතු වූ collection එකකි.\
array එකක් සෑදීමේදී එහි පවතින data වල type එක හා එහි length එක අනිවාර්යයෙන්ම සටහන් කල යුතුය. `[TYPE; length]`. array එකක length එක compile කරන අවස්ථාවේදී අනිවාර්යය වේ.
```rust
let array_1: [i32;5] = [1, 2, 3, 4, 5];
println!("array-1 is: {:?}", array_1);
// array-1 is: [1, 2, 3, 4, 5]
```
array එකක සියළුම අගයන් සමාන නම් එය පහත පරිදි පහසුවෙන් සෑදිය හැක.
```rust
let array_2: [char; 7] = ['a'; 7];
println!("array-2 is: {:?}", array_2);
// array-2 is: ['a', 'a', 'a', 'a', 'a', 'a', 'a']
```
array එකකින් එහි index එක භාවිතා කර කිසියම් element ලබා ගැනීම පහත පරිදි සිදුකල හැක.
```rust
println!("3rd element of the array_1: {}", array_1[2]);
// 3rd element of the array_1: 3
```

array එකක length එක ලබා ගැනීම පහත පරිදි සිදුකල හැක.
```rust
println!("length of the array_1: {}", array_1.len());
// length of the array_1: 5
```

## Slice
Slice එකක් array එකකට සමාන වුවත් compile time එකේදී එහි length එක අවශ්‍ය නොවේ.\
slice එකක් සෑදීම පහත පරිදි සිදුකල හැක. `&[TYPE]`
```rust
let slice_1: &[char] = &['a', 'p', 'p', 'l', 'e'];
println!("slice_1 is: {:?}", slice_1);
// slice_1 is: ['a', 'p', 'p', 'l', 'e']
```
ඕනෑම array එකක් ඊට ඉදිරියෙන් `&` යෙදීමෙන් slice එකක් බවට පත් කල හැක.
```rust
let array_3: [i32; 10] = [1, 4, 9, 16, 25, 36, 49, 64, 81, 100];
let slice_3: &[i32] = &array_3;
println!("slice_3 is: {:?}", slice_3);
// slice_3 is: [1, 4, 9, 16, 25, 36, 49, 64, 81, 100]
```

array එකක යම් කොටසක් පමණක් පහත ආකාරයට slice එකක් බවට පරිවර්ථනය කල හැක.
- එහි format එක වන්නේ `[starting_index..ending_index]`
- starting_index එක slice එකේ පළමු element එක වේ.
- ending_index එකට පෙර element එක slice එකේ අවසාන element එක වේ.
```rust
let slice_part: &[i32] = &array_3[3..7];
println!("slice_part is: {:?}", slice_part);
// slice_part is: [16, 25, 36, 49]
```
පහත ආකාරයටද array එකක් slice කිරීම සිදුකල හැක.
```rust
let playground: [char; 10] = ['p','l','a','y','g','r','o','u','n','d'];
let playgorund_slice: &[char] = &playground[..];
let gorund_slice: &[char] = &playground[4..];
let play_slice: &[char] = &playground[..4];
println!("playground slice is : {:?}", playgorund_slice);
println!("ground slice is : {:?}", gorund_slice);
println!("play slice is : {:?}", play_slice);
// playground slice is : ['p', 'l', 'a', 'y', 'g', 'r', 'o', 'u', 'n', 'd']
// ground slice is : ['g', 'r', 'o', 'u', 'n', 'd']
// play slice is : ['p', 'l', 'a', 'y']
```

slice එකක length එක හා index එකක් භාවිතා කර element එකක් ලබා ගැනීම array එකක ආකාරයටම සිදුකල හැක.
```rust
println!("2nd element of the slice_part: {}", slice_part[1]);
println!("length of the slice_part: {}", slice_part.len());
// 2nd element of the slice_part: 25
// length of the slice_part: 4
```








