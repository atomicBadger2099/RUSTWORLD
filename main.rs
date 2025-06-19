//THIS IS A TEST PROGRAM FOR VARIABLES IN RUST

fn main() {
    println!(" \n");
    println!("Welcome to the variable test program in Rust.\n");

// SOME VARIABLES!
    let first_var = 1;
    let (var_2, var_3) = (22, 33);
    const VAR_MONSTER: u32 = 777;
    let fourth_var = 3.14;
    let (letter_1, letter_2) = ('Y', 'N');
    let (result_1, result_2) = (true, false);

    println!("The first variable is: {}", first_var);
    println!("The second variable is: {}", var_2);
    println!("The third variable is: {}", var_3);
    println!(" \n");
    println!("The first CONSTANT is: {}", VAR_MONSTER);
    println!(" \n");
    println!("The fourth variable is: {}", fourth_var);
    println!("The two characters are: {} and {}", letter_1, letter_2);
    println!("BOOLEAN results are: {} and {}", result_1, result_2);
    println!(" \n");
}
