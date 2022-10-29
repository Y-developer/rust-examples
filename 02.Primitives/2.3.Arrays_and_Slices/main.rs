fn main() {
    println!("Create an array");
    let array_1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array-1 is: {:?}", array_1);

    println!("");
    println!("Array with same values");
    let array_2: [char; 7] = ['a'; 7];
    println!("array-2 is: {:?}", array_2);

    println!("");
    println!("Get element in array");
    println!("3rd element of the array_1: {}", array_1[2]);

    println!("");
    println!("Get length of array");
    println!("length of the array_1: {}", array_1.len());

    println!("");
    println!("Create a slice");
    let slice_1: &[char] = &['a', 'p', 'p', 'l', 'e'];
    println!("slice_1 is: {:?}", slice_1);

    println!("");
    println!("Convert entire array to slice");
    let array_3: [i32; 10] = [1, 4, 9, 16, 25, 36, 49, 64, 81, 100];
    let slice_3: &[i32] = &array_3;
    println!("slice_3 is: {:?}", slice_3);

    println!("");
    println!("Convert a part of the array to slice");
    let slice_part: &[i32] = &array_3[3..7];
    println!("slice_part is: {:?}", slice_part);

    println!("");
    println!("Way to slice");
    let playground: [char; 10] = ['p', 'l', 'a', 'y', 'g', 'r', 'o', 'u', 'n', 'd'];
    let playgorund_slice: &[char] = &playground[..];
    let gorund_slice: &[char] = &playground[4..];
    let play_slice: &[char] = &playground[..4];
    println!("playground slice is : {:?}", playgorund_slice);
    println!("ground slice is : {:?}", gorund_slice);
    println!("play slice is : {:?}", play_slice);

    println!("");
    println!("Get element and length from slice");
    println!("2nd element of the slice_part: {}", slice_part[1]);
    println!("length of the slice_part: {}", slice_part.len());
}
