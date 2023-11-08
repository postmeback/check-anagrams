use std::io;

fn main() {
   let mut first_string = String::new();

   let mut second_string = String::new();

   println!("Enter 1st string");
   io::stdin().read_line(&mut first_string).expect("Invalid Input");

   println!("Enter 2nd string");
   io::stdin().read_line(&mut second_string).expect("Invalid Input");

   let result = are_anagrams(&first_string, &second_string);

   println!("The result is {}", result);
}

fn are_anagrams(first_string: &str, second_string: &str) -> bool {
  
  let mut sorted_str1: Vec<char> = first_string.chars().collect();

  let mut sorted_str2: Vec<char> = second_string.chars().collect();

  return sorted_str1.sort() == sorted_str2.sort();

}
