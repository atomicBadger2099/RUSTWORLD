fn main() {
    println!("=== Rust Variables and Primitive Types Demo ===\n");

    // Integer Types
    println!("--- Integer Types ---");
    let i8_val: i8 = -128;
    let i16_val: i16 = -32768;
    let i32_val: i32 = -2147483648;
    let i64_val: i64 = -9223372036854775808;
    let i128_val: i128 = -170141183460469231731687303715884105728;
    let isize_val: isize = -100;

    let u8_val: u8 = 255;
    let u16_val: u16 = 65535;
    let u32_val: u32 = 4294967295;
    let u64_val: u64 = 18446744073709551615;
    let u128_val: u128 = 340282366920938463463374607431768211455;
    let usize_val: usize = 100;

    println!("i8: {}", i8_val);
    println!("i16: {}", i16_val);
    println!("i32: {}", i32_val);
    println!("i64: {}", i64_val);
    println!("i128: {}", i128_val);
    println!("isize: {}", isize_val);
    println!("u8: {}", u8_val);
    println!("u16: {}", u16_val);
    println!("u32: {}", u32_val);
    println!("u64: {}", u64_val);
    println!("u128: {}", u128_val);
    println!("usize: {}", usize_val);

    // Floating Point Types
    println!("\n--- Floating Point Types ---");
    let f32_val: f32 = 3.14159;
    let f64_val: f64 = 2.718281828459045;
    println!("f32: {}", f32_val);
    println!("f64: {}", f64_val);

    // Boolean Type
    println!("\n--- Boolean Type ---");
    let bool_true: bool = true;
    let bool_false: bool = false;
    println!("bool (true): {}", bool_true);
    println!("bool (false): {}", bool_false);

    // Character Type
    println!("\n--- Character Type ---");
    let char_ascii: char = 'A';
    let char_unicode: char = '🦀';
    let char_emoji: char = '😊';
    println!("char (ASCII): {}", char_ascii);
    println!("char (Unicode): {}", char_unicode);
    println!("char (Emoji): {}", char_emoji);

    // Unit Type
    println!("\n--- Unit Type ---");
    let unit_val: () = ();
    println!("unit type: {:?}", unit_val);

    // Mutable Variables Examples
    println!("\n--- Mutable Variables ---");
    let mut mutable_counter = 0;
    println!("Initial counter: {}", mutable_counter);
    mutable_counter += 1;
    println!("After increment: {}", mutable_counter);
    mutable_counter *= 5;
    println!("After multiplication: {}", mutable_counter);

    // Multiple mutable variables
    let mut x = 10;
    let mut y = 20;
    let mut z = 30;
    println!("Initial values: x={}, y={}, z={}", x, y, z);
    
    x += y;
    y *= 2;
    z -= 5;
    println!("After operations: x={}, y={}, z={}", x, y, z);

    // Mutable string example
    let mut greeting = String::from("Hello");
    println!("Original greeting: {}", greeting);
    greeting.push_str(", World!");
    println!("Modified greeting: {}", greeting);

    // Variable Shadowing Examples
    println!("\n--- Variable Shadowing ---");
    
    // Basic shadowing
    let shadow_var = 5;
    println!("First shadow_var: {}", shadow_var);
    let shadow_var = shadow_var + 1;
    println!("Second shadow_var: {}", shadow_var);
    let shadow_var = shadow_var * 2;
    println!("Third shadow_var: {}", shadow_var);

    // Shadowing with type change
    let spaces = "   ";
    println!("spaces as string: '{}'", spaces);
    let spaces = spaces.len();
    println!("spaces as number: {}", spaces);

    // Shadowing in different scopes
    let outer_var = 100;
    println!("Outer scope: {}", outer_var);
    {
        let outer_var = 200;
        println!("Inner scope: {}", outer_var);
        let outer_var = "now a string";
        println!("Inner scope (shadowed again): {}", outer_var);
    }
    println!("Back to outer scope: {}", outer_var);

    // Shadowing with calculations
    let num = 8;
    let num = num + 2;
    let num = num * 3;
    let num = format!("The result is: {}", num);
    println!("Final shadowed value: {}", num);

    // Constants (for comparison with variables)
    println!("\n--- Constants (for comparison) ---");
    const MAX_POINTS: u32 = 100_000;
    const PI: f64 = 3.14159265359;
    println!("Constant MAX_POINTS: {}", MAX_POINTS);
    println!("Constant PI: {}", PI);

    println!("\n=== Demo Complete ===");
}
