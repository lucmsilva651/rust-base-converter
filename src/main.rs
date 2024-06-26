use std::env;

fn main() {
  //collect arguments
  let num_inf: Vec<String> = env::args().collect();

  if num_inf.len() < 2 {
    println!("Return any number to be converted.");
  }
   
  for num_arg in &num_inf[1..] { //pick 2nd arg bcs 1st is the executable itself
    if let Ok(num_int) = num_arg.parse::<u64>() { //parse arg into u64  
      println!("Base 10 (normal):  {}",   num_int);
      println!("Base 2 (binary):   {:b}", num_int);
      println!("Base 8 (octal):    {:o}", num_int);
      println!("Base 16 (hexa):    {:x}", num_int);
    } else {
      println!("Your number cannot be converted. Try another.");
    }
  }
}