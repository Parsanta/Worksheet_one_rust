// use std::io;

//task_1 hello_world
// fn main(){
//     println!("Hello World");
// }

//task_2  personalized greeting
// fn main(){
//     let mut name = String::new();
//     println!("enter your name:");
//     io::stdin().read_line(&mut name);
//     println!("Hello {name},Nice to see you");

// }

//task_3 Celsius to Fahrenheit or wiseversa
// fn main() {
//     let mut inp = String::new();
//     let mut scale = String::new();
//     println!("Select your input type (Celsius: C or Fahrenheit: F): ");
//     io::stdin().read_line(&mut scale).expect("Failed to read");

//     println!("Enter temperature value: ");
//     io::stdin().read_line(&mut inp).expect("Failed to read");

//     let temperature: f64 = inp.trim().parse().expect("Invalid");
    
//     if scale.trim() == "C"{
//         let fahrenheit = (temperature * 9.0 / 5.0) + 32.0;
//         println!("{}°C is equal to {}°F", temperature, fahrenheit);
//     } else if scale.trim() == "F" {
//         let celsius = (temperature - 32.0) * 5.0 / 9.0;
//         println!("{}°F is equal to {}°C", temperature, celsius);
//     } else {
//         println!("Invalid input type. Please enter C or F.");
//     }
// }

// task_4 Factorial
// fn main(){
//     let num = 5;
//     println!("fact {}",fact(num))
// }
// fn fact(x:i32)->i32{
//     if x == 0 {
//         1
//     }else{
//         x*fact(x-1)
//     }
     
// }

// task_5 ODD?EVEN
// fn main(){
//     let a = 5;
//     if a%2==0{
//         println!("num is even");
//     }else{
//         print!("num is odd");
//     }
// }

// task_6 sum of all numbers
// fn main(){
//     let mut inp=String::new();
//     let mut sum=0;
//     println!("enter a number :");
//     io::stdin().read_line(&mut inp).expect("invalid");

//     let inp : i32 = inp.trim().parse().expect("invalid");
//     for i in 0..=inp{
//         sum+=i;
//     }
//     println!("sum till {} is {}",inp,sum);
// }

// task_7 
// fn main(){
//     for i in 0..=100{
//         if (i%3==0 && i%5==0){
//             println!("FrizzBuzz");
//         }else if i %3==0{
//             println!("Frizz");
//         } else if i % 5 ==0{
//             println!("Buzz");
//         } else{
//             println!("{i}");
//         }
//     }
// }

// task_8 palindrome String
// fn main() {
//     let inp = ['a', 'b', 'a'];
//     let length = inp.len();
//     let mut i = 0;

//     while i < length / 2 {
//         if inp[i] != inp[length - i - 1] {
//             println!("Not a palindrome");
//             return;
//         }
//         i += 1;
//     }

//     println!("Palindrome");
// }

// task_9 Guess game
// fn main(){

// }

// task_10 calculator
// fn main(){
//     let num1 = 3;
//     let num2 = 4;
//     let op = "+";
//     if (op=="+"){
//         print!("{}",num1+num2);
//     }
// }

// // task_11 largest num in array
// fn main(){
//     let arr= [1,10,15,6,7];
//     let mut max = 0;
//     for i in arr{
//         if max < i{
//             max=i;
//         }
//     }
//     print!("max value is {}",max);
// }

// task_12 reversed string
// fn main(){
//     let mut inp = String::new();
//     let mut reversed=String::new();
//     println!("enter a string:");
//     io::stdin().read_line(&mut inp).expect("invalid input");
//     let input = inp.chars();
//     reversed = input.rev().collect();
//     print!("reversed : {}",reversed);
// }


//task_13 fibonacci series

// fn main() {
//     let num = 10;
//     let mut a = 0;
//     let mut b = 1;
//     let mut orig = num;

//     while orig>0{
//         println!("{a}");
//         let next = a+b;
//         a=b;
//         b=next;
//         orig-=1;
//     }

// }

// task_14 print till n
// fn main(){
//     let n = 10;
//     for i in 0..=n{
//         println!("{}",i)
//     }
// }

// task_15


//task_17  prints its first and last character using string slicing
// fn main(){
//     let name = "hello";
//     let f_char = &name[0..1];
//     let L_char=&name[name.len()-1..];

//     println!("first character {} last character {}",f_char,L_char);
// }


//task_18 number of times the substring appears in the string.

// fn main() {
//     let word = String::from("abcabcababb");
//     let sub = String::from("abc");
//     let mut count = 0;

//     for i in 0..=word.len()-sub.len() {
//         let mut char_count = 0;
//         for j in 0..sub.len() {
//             if word.as_bytes()[i + j] == sub.as_bytes()[j] {
//                 char_count += 1;
//             } else {
//                 break;
//             }
//         }
//         if char_count == sub.len() {
//             count += 1;
//         }
//     }

//     println!("The substring '{}' appears {} times in the main string.", sub, count);
// }



