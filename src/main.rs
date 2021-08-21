struct Car {
    make: String,
    model: String,
    year: u32,
}

struct Point2D(u32, u32);

struct Person {
    name: String,
    age: u8,
    likes_oranges: bool
}

enum WebEvent {
    PageLoad,
    PageUnload,
    Keypress(char),
    Paste(String),
    Click {x:i64, y:i64},
}

enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let x = 0.5;
    println!("x: {}", x);

    let array: [u32; 3] = [1, 2 , 3];

    println!("array: {:?}", array);

    let tuple: (String, i32, bool) = (String::from("Haja"), 29, true);

    println!("Info about => {} is {} years old", tuple.0, tuple.1);

    println!("sum of [2, 4 ,6] = {}", array_sum([2, 4, 6]));

    let car = Car {
        make: String::from("Honda"),
        model: String::from("Accord"),
        year: 2015,
    };

    println!("car make: {}", car.make);

    let origin = Point2D(100, 200);

    println!("Point contains {:?} and {:?}", origin.0, origin.1);

    let Point2D(x, y) = origin;
    println!("Point contains {:?} and {:?}", x, y);

    let person = Person {
        name: String::from("Adam"),
        likes_oranges: true,
        age: 25,
    };

    println!("Person name is: {}", person.name);

    // ENUMS

    let quit = WebEvent::Keypress('q');

    let something = Some(1);

    // CONDITIONS

    if 1 == 2 {
        println!("math is broken")
    } else {
        println!("everything is fine");
    }

    let formal = true;
    let greeting = if formal {
        println!("Good evening");
    } else {
        println!("Hey, friend");
    };

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4 or 3");
    };

    let boolean = true;

    let binary = match boolean {
        false => 0,
        true => 1,
    };

    // LOOPS

    let mut i: i32 = 1;
    let something: i32 = loop {
        i *= 2;
        if i > 100 {
            break i;
        }
    };

    assert_eq!(something, 128);

    let mut counter = 0;

    while counter < 10 {
        println!("Hello");
        counter += 1;
    }

    for item in 0..5 {
        println!("{}", item * 2);
    }

    // Error handler
}

fn array_sum(array: [i32; 3]) -> i32 {
    array.iter().sum()
}




