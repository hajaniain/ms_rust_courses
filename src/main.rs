fn main() {
    let x = 0.5;
    println!("x: {}", x);

    let array: [u32; 3] = [1, 2 , 3];

    println!("array: {:?}", array);

    let tuple: (String, i32, bool) = (String::from("Haja"), 29, true);

    println!("Info about => {} is {} years old", tuple.0, tuple.1);

    println!("sum of [2, 4 ,6] = {}", array_sum([2, 4, 6]));
}

fn array_sum(array: [i32; 3]) -> i32 {
    array.iter().sum()
}


