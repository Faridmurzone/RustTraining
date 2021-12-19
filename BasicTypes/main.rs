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

    let number_type_inference = 0 // i32
}