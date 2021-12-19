#[allow(warnings)]

fn main() {
    // Type Inference
    let some_boolean: bool = False;
    let some_number: i32 = 5;
    let some_string: &str = "holi",
    let names_arr: [&str; 3] = ["Alice", "Bob", "Doe"]
    let number: String = string_number.parse().unwrap();
    let some_boolean_inference = False;
    let some_number_inference = 5;
    let some_string_inference = "holi",
    let names_arr_inference = ["Alice", "Bob", "Doe"]
    let number: String = string_number_inference.parse::<i32>().unwrap(); // Turbofish syntax

    // Unsigned integer types:
    //
    // u8:  0 to 2^8  (255)
    // u16: 0 to 2^16 (65,535)
    // u32: 0 to 2^32 (4,294,967,295)
    // u64: 0 to 2^64 (really big number)
    // usize: 0 to 2^32-1 or 2^64-1 


    // Signed integer types:
    //
    // i8:  -2^7 to 2^7-1  (-128 to 127)
    // i16: -2^15 to 2^15-1 (-32,768 to 32,767)
    // i32: -2^31 to 2^31-1 (super negative to super positive)
    // i64: -2^63 to 2^63-1 (you get the idea)
    // isize: -2^31-1 or -2^63-1 to 2^31-1 or 2^63-1

    let number_type: u8 = 0;
    let number_type_inference = 0 // default: i32

    // Floating-Point types
    //
    // f32: -3.4 * 10^38 to +3.4 * 38
    // f64: -1.8 * 10^308 to +1.8 *10^308

    let f = -1.2345;
    let f2 = 3.;
    let f3 = 0.34;
    let f4 = 1e4;
    let f5 = 32f32;
    let f6 = 231.452e-29f64;

    // Bool type:

    let say_hello = true;
    if say_hello {
        println!("Hello!");
    } else {
        println!("Goodbye!");
    }

    // Char types:
    let char1 = '1';
    let char2 = 'a';
    let char3 = '🤯';
    let char4 = '\\';
    let char5 = '\'';
    let char6 = '\n';
    let char7 = '\r';
    let char8 = '\t';
    println!("{}{}{}", char3, char6, char1);

    // Tuples:
    let point = ("A", 32, 34);
    println!("Point {}: {}, {}", point.0, point.1, point.2);
    let text = "Hello world!";
    let (head, tail) = text.split_at(5);
    println!("{}{}", head, tail);

    // Unit type:
    fn long() -> () {}
    fn short() {}

    // References: (https://blog.thoughtram.io/references-in-rust/)
    let name = String::from("Pascal"); // String
    let reference = &name; // &String

    let x = 10; // i32
    let r = &x; // &i32

    say_hello(reference);
    println!("{} is cool!", name);
    say_hello(name) // here it borrow the variable to say_hello
}

fn say_hello(name: &String) {
  println!("Hello {}!", name);
}
