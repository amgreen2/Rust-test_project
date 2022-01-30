use std::io;

fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of inner x is: {}", x);
    }
    println!("The value of x is: {}", x);
//tuples 
    let tup : (String, u64) = ("green2clean".to_string(), 7401);
    println!("The username is: {}", tup.0.to_string());
    println!("The ID is: {}", tup.1);
    println!("The full is: {}{}", tup.0.to_string(), tup.1);
//arrays
    let arr : [String; 3] = ["green2clean".to_string(), "scansenator".to_string(), "lordpeepee".to_string()];
    let first = arr[0].to_string();
    println!("Friend one is: {}", first);
    println!("Friend two is: {}", arr[1]);
    println!("Friend three is: {}", arr[2]);
    
    let arr2 = ["bee", "ant", "wasp"];
    println!("Please enter a number 1-3");
    let mut entry = String::new();

    io::stdin()
        .read_line(&mut entry)
        .expect("Failed");
    
    let entry: usize = entry 
        .trim()
        .parse()
        .expect("Number entered was invalid format");

    let mod_entry: usize = sub_one(entry);
    let element = arr2[mod_entry];//subtract one to account for zero based index
    print_loop(arr2);
    println!(
        "You entered {} so you get: {}",
        entry, element
    );
//functions
    if mod_entry == 0 {//if entry is 3, enter else condition
        bee("Bees are", " curious");
    } else if mod_entry == 1 {
        ant("Ants are", " strong");
    } else if mod_entry == 2 {
        wasp("Wasps are", " scary");
    }
    countdown();
    mut_string();
}
fn ant(a: &str, b: &str) {
    println!("Hello! {}{}", a, b);
}
fn bee(a: &str, b: &str) {
    println!("Hello! {}{}", a, b);
}
fn wasp(a: &str, b: &str) {
    println!("Hello! {}{}", a, b);
}
fn sub_one(x: usize) -> usize {
    x - 1
}
fn print_loop(list: [&str; 3]) {
    loop {
        println!("1={}, 2={}, 3={}", list[0], list[1], list[2]);
        break;
    }
}// test while loop
fn countdown() {
    for number in (1..5).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}
fn mut_string() {//test print string
    let mut s = String::from("hello");
    let len = calculate_length(&s);
    s.push_str(", world");
    println!("{} length {} ", s, len);
}//set string reference to be mutable
fn calculate_length(s: &String) -> usize {
    s.len()
}
