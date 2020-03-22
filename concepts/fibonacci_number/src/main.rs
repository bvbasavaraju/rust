use std::io;

#[allow(non_snake_case)]
fn main() 
{
  let count: i32;
  loop
  {
    println!("\nEnter number of Fibonacci number to be generated:");
    let mut numOfFibonacciNumbersInStr = String::new();
    io::stdin().read_line(&mut numOfFibonacciNumbersInStr)
               .expect("Filed to read");

    count = match numOfFibonacciNumbersInStr.trim().parse()
    {
      Ok(num) => num,
      Err(_) =>
      {
        println!("Invalid input");
        continue;
      }
    };

    break;
  }

  let mut nthNumber: i32 = 1;
  nthNumber = GenerateNthFibonacciNumberInRecursion(count, 0, &mut nthNumber);
  println!("Nth Fibonacci number is: {}", nthNumber);

  nthNumber = GenerateNthFibonacciNumber(count);
  println!("Nth Fibonacci number is: {}", nthNumber);
  return;
}

// 0 1 1 2 3 5 8 13 21 34 55 89
#[allow(non_snake_case)]
fn GenerateNthFibonacciNumberInRecursion(count: i32, prevNum: i32, number: &mut i32) -> i32
{
  static mut CURRENT_COUNT: i32 = 1;

  let mut retVal: i32 = *number;
  unsafe
  {
    if CURRENT_COUNT >= count
    {
      return retVal;
    }
    CURRENT_COUNT = CURRENT_COUNT + 1;
  }
  retVal = GenerateNthFibonacciNumberInRecursion(count, retVal, &mut (prevNum + retVal));
  return retVal;
}

//Using looping!
#[allow(non_snake_case)]
fn GenerateNthFibonacciNumber(count: i32) -> i32
{
  let mut currentCount = 1;

  let mut firstNum = 0;
  let mut secondNum = 1;
  let mut result: i32 = 0;
  while currentCount < count
  {
    result = firstNum + secondNum;

    firstNum = secondNum;
    secondNum = result;
    currentCount += 1;
  }

  return result;
}