use std::io;

//If using non snake casing, then make sure to add this warning excluder statement!
#[allow(non_snake_case)]
fn main() 
{
  //Notice that "mut" is not used here.
  //Reason is, it is not assigned even once! 
  //so code in line#20 is valid.println!
  //Assigning value for variable once is still valid in a scope of the variable
  let tempInFahrenheit: f32;
  loop
  {
    println!("\n\nEnter the Temperature in Fahrenheit");

    let mut dataFromUser = String::new(); 
    io::stdin().read_line(&mut dataFromUser)
                    .expect("Failed to read input");

    tempInFahrenheit = match dataFromUser.trim().parse()
    {
      Ok(num) => num,
      Err(_) =>
      {
        println!("Entered data is not a valid input");
        continue;
      }
    };

    break;
  }

  let tempInDegCelsius: f32 = ConvertToDegCelsius(tempInFahrenheit);
  println!("Temperature in Fahrenheit is: {}", tempInFahrenheit);
  println!("Temperature in DegCelsius is: {}", tempInDegCelsius);

  //Making use of Shadow concept for the variable tempInFahrenheit
  let tempInFahrenheit: f32 = ConvertToFahrenheit(tempInDegCelsius);
  println!("Temperature in Fahrenheit converted is: {}", tempInFahrenheit);
  return;
}

#[allow(non_snake_case)]
fn ConvertToDegCelsius(tempInFahrenheit: f32) -> f32
{
  const ZERO_DEGREE_IN_FAHRENHEIT: f32 = 32.0;

  let retVal: f32 = (tempInFahrenheit - ZERO_DEGREE_IN_FAHRENHEIT) * (5.0 / 9.0);
  return retVal;
}

#[allow(non_snake_case)]
fn ConvertToFahrenheit(tempInDegCelsius: f32) -> f32
{
  const ZERO_DEGREE_IN_FAHRENHEIT: f32 = 32.0;

  let retVal: f32 = (tempInDegCelsius * (9.0 / 5.0)) + ZERO_DEGREE_IN_FAHRENHEIT;
  return retVal;
}