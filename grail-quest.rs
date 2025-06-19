// GRAIL QUEST PROGRAM
use std::io;

fn main() {
    // OPENING
    println!(" ");
    println!("---------------------------------------------------------------------");
    println!("THE GRAIL QUEST PROGRAM");
    println!("by Dwayne Brock");
    println!("May 17, 2025");
    println!("---------------------------------------------------------------------");
    println!(" ");
    
    println!("\nBEGIN PROGRAM.\n");
    println!("\nThe guardian challenges you with three questions.\n");
    
    // COLLECT INFO 1
    println!("\nWHAT IS YOUR NAME?");
    println!("(The brave knight should answer and hit return, please.)");
    let mut name_g = String::new();
    io::stdin().read_line(&mut name_g).expect("Failed to read line");
    let name_g = name_g.trim(); // Remove newline character
    
    // COLLECT INFO 2
    println!("\nWHAT IS YOUR QUEST?");
    println!("(The brave knight should answer and hit return, please.)");
    let mut quest_g = String::new();
    io::stdin().read_line(&mut quest_g).expect("Failed to read line");
    let quest_g = quest_g.trim(); // Remove newline character
    
    // COLLECT INFO 3
    println!("\nWHAT IS YOUR FAVORITE COLOR?");
    println!("(The brave knight should answer and hit return, please.)");
    let mut color_g = String::new();
    io::stdin().read_line(&mut color_g).expect("Failed to read line");
    let color_g = color_g.trim(); // Remove newline character
    
    println!(" ");
    
    // ANSWERS
    println!("\nYOU HAVE ANSWERED:");
    println!("\n\tNAME: {}", name_g);
    println!("\tQUEST: {}", quest_g);
    println!("\tCOLOR: {}\n", color_g);
    println!(" ");
    
    // CLOSING
    println!("YOU HAVE CHOSEN WISELY, BRAVE KNIGHT.");
    println!(" ");
    println!("Please turn left at the deadly rabbit and right at the giant horned monster.");
    println!(" ");
    
    println!("---------------------------------------------------------------------");
    println!("END OF PROGRAM.");
    println!("you have reached the end. Now go do something happy.");
    println!("---------------------------------------------------------------------");
    println!(" ");
}
