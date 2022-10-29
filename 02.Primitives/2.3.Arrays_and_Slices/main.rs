fn main() {
    println!("create an array");
    let array_1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array-1 is: {:?}", array_1);

    println!("");
    println!("array with same values");
    let array_2: [char; 7] = ['a'; 7];
    println!("array-2 is: {:?}", array_2);

    println!("");
    println!("get element in array");
    println!("3rd element of the array_1: {}", array_1[2]);

    println!("");
    println!("get length of array");
    println!("length of the array_1: {}", array_1.len());

    println!("");
    println!("create a slice");
    let slice_1: &[char] = &['a', 'p', 'p', 'l', 'e'];
    println!("slice_1 is: {:?}", slice_1);

    println!("");
    println!("convert entire array to slice");
    let array_3: [i32; 10] = [1, 4, 9, 16, 25, 36, 49, 64, 81, 100];
    let slice_3: &[i32] = &array_3;
    println!("slice_3 is: {:?}", slice_3);

    println!("");
    println!("convert a part of the array to slice");
    let slice_part: &[i32] = &array_3[3..7];
    println!("slice_part is: {:?}", slice_part);

    println!("");
    println!("get element and length from slice");
    println!("2nd element of the slice_part: {}", slice_part[1]);
    println!("length of the slice_part: {}", slice_part.len());
}
