fn main() {
    println!("Hello test from rust!");
    let x: i32 = -67;
    let e: f32 = 2.712;
    let mut is_summer: bool = false;
    let last_letter: char = 'z';
    println!("Hello, x is: {}", x);
    println!("Hello, y is: {}", e);
    println!("Hello, z is: {} {}", is_summer, last_letter);
    is_summer = true;
    println!("hi {}",is_summer);
    is_summer = false;
    println!("hi {}",is_summer);
    let letters: [char;4] = ['a','b','c','d'];
    println!("letters: {:?}",letters);
    let names:[&str;3]=["Tony","Jalal","Philip"];
    println!("workers: {:?}",names);
    println!("first: {}",names[0]);
    println!("last: {}",names[2]);
    let human:(String, i32, bool) = ("Tony".to_string(),15,true);
    println!("human: {:?}",human);
    print!("Here are the stats:");
    whats_a_human(human.0,human.1, human.2);
    let number_slices:&[i32] = &[1,2,3,4];
    println!("first num & second num: {} {}", number_slices[0], number_slices[1]);
    let books:&[String] = &["lord of the rings".to_string(),"harry potter".to_string()];
    println!("books: {:?}",books);
    let mut string:String = "Hello ".to_string();
    string.push_str("world!");
    println!("hi: {}",string);
    let new_string:String = "test".to_string();
    let new_slice:&str = &new_string[0..2];
    println!("new slice: {:?}, string form: {:?}",new_slice, new_string);
    another_function();
    let another_string:String = "immutable".to_string();
    println!("the length of another string is {} and pi is {}",borrow_reference(&another_string), PI_VALUE);
    let mut rand:i32 = 67;
    rand+=2;
    rand*=2;
    println!("new rand {}",rand);
    const FINAL_SIZE:usize = "hi".len();
    println!("og string {}, final size: {}",another_string,FINAL_SIZE);
    let shadowing = "hello there";
    println!("hi {}",shadowing);
    let shadowing = shadowing.len();
    println!("size: {}",shadowing);
}

fn another_function() {
    println!("called from another function");
}

fn whats_a_human(name:String, age:i32,hacker:bool) {
    println!("Name: {}, age: {}, is a hacker: {}", name, age, hacker);
}

fn borrow_reference(string:&String)->usize {
    String::from(string.to_string()).len()
}

const PI_VALUE:f64 = 3.141592653589;