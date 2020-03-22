use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn guess_game()
{
  loop
  {
    println!("\n\nEnter your guess between 1 to 10:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
              .expect("Error reading the given input");

    let input_: u32 = match input.trim().parse()
    {
      Ok(num) => num,       // in case of input is a number then that should be returned so that value can be assigned to input_. 
                            // so, "num" is the result of parse(). num will be returned and assigned to input_
      Err(_) => continue,   // here all errors are captured by using "_" in Err
    };
    println!("Entered number is: {}", input_);

    //Default type of number type in rust is i32!
    //Hence random_num is of type i32, as type is not specified here!
    let random_num = rand::thread_rng().gen_range(1, 10);
    println!("Random num generated is: {}", random_num);

    // match is similar to switch in C++!!
    // using match, return of cmp can be used to switch to specific piece of code
    match input_.cmp(&random_num)
    {
      Ordering::Less => println!("NOPE!!! number entered is Less"),
      Ordering::Greater => println!("NOPE!!! number entered is Greater"),
      Ordering::Equal =>
      {
        println!("*** Oops! you guessed exact same number ***\n");
        break;
      }
    }
  }
}

fn main()
{
  guess_game();

  return;
}