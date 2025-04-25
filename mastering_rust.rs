fn main() {
    let _x = 3.14;
    let _y = 20;
    let mut message = String::from("Hello, ");
    let _fruit = "apple";
    let fruit1 = "banana";
 
 println!("_x is {_x}");
 println!("_y is {_y}");
 message.push_str("Rust");
 println!("Message is {message}");
 
 if _x < _y as f64 { 
  println!("{_x} is less than {_y}");
 }else if _x == _y as f64 {
  println!("{_x} is equal to {_y}");
 }
 else {
  println!("{_x} is greater than {_y}");   
 }
 
 match fruit1 {
     "apple" => println!("Dorikindoch Yapple!"),
     "banana" => println!("Aratipandu dorikindi, hmm!"),
     _ => println!("Yado oka pandu ivvu, please"),
 }
 
 let mut counter = 0; 
 
 loop {
     println!("Counter is {}", counter);
     counter += 1;
     
     if counter >=5 {
        println!(" Counter 5 vachindi, Break cheyyara");
         break;
     }  
 }
 
 let numbers = [1, 2, 3, 4, 5];
 
 for number in numbers.iter() {
 //for number in 1..6 another way to write for loop with range
     println!("Number:{}", number);
 }
 let sum = add(10,56);
 println!("Sum :{}", sum);
 
 greet("Seethamma, Ramayya, Lakshmana");
 greet("Hanuman");
 
 simpson_module::homer_is_public();
// simpson_module::bart_is_not_public();
}
 fn add(a:i32, b:i32) -> i32 {
      a+b
 }
 fn greet(name:&str) {
     println!("Hello, {}", name);
 }

mod simpson_module {
    pub fn homer_is_public() {
        println!("Homer is public man");
    }
    
    fn bart_is_not_public() {
        println!("Bart is not public");
    }
}
