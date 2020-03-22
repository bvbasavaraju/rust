use std::fmt; // For formatting the rectangle

#[derive(Debug)]    // For enabling Debug; another approach is defining trait std::fmt::Debug
struct Rectangle
{
  width : i32,
  height : i32,
}

//Here Rectangle is considered as namespace in C++
impl Rectangle
{
    //Methods
    fn area(&self) -> i64   // Here notice that return type is i64, where as the width and height are of type i32
    {
        // notice the use of i64::from() on the return of the multiplication
        // also, ";" is not used in the return line, because result to be returned. if ";" is used, then () "empty tuple" will be returned
        return i64::from(self.width * self.height)
    }

    fn is_square(&self) -> bool
    {
        return self.width == self.height
    }

    fn print_area(&self)
    {
        println!("{} X {} ==> {} ; inside print_area(), called area() method/api", self.width, self.height, self.area());
    }

    //Related functions - These are like static functions in c++
    fn new(w : i32, h : i32) -> Rectangle
    {
        return Rectangle {width: w, height: h}
    }

    fn create(w: i32, h: i32) -> Rectangle
    {
        return Rectangle {width: w, height: h}
    }
}

//It is basically inheriting Rectangle from the class/namespace std::fmt. similar to inheritance in C++.
//So, now define the pure virtual functions
impl fmt::Display for Rectangle
{
    //fmt is the pure virtual functions which needs to be defined
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        //note the usage of f, that is a Formatter in the writeln!() macro
        //As fmt function has to return the fmt::Result, there is no ";" at the end of the macro writeln!()
        writeln!(f, "Rectangle.width : {}\nRectangle.height : {}\nRectangle.area : {}\nRectangle is square: {}", self.width, self.height, self.area(), self.is_square())
    }
}

fn main()
{
  println!("\nwidth X height ==> Area");
  
  println!("Rectangle r1:");
  let r1 : Rectangle = Rectangle {width : 10, height: 10};  // here the r1 object has been created in stack. and is immutable
  println!("{} X {} ==> {} ", r1.width, r1.height, r1.width * r1.height);
  println!("{} X {} ==> {} ; by calling method area", r1.width, r1.height, r1.area());
  println!("{:?}\n", r1);   //For Debug, structure must derive Debug use keywords "#derive[(Debug)]"
  println!("{:#?}\n", r1);   //by adding "#" inside {}, it will be pretty display, basically with new lines
  println!("{}", r1);

  // Rectangle 2 
  println!("----------------------------\n\nRectangle r2:");
  let r2 = Rectangle::new(20, 20);
  r2.print_area();
  println!("{}", r2);

  println!("----------------------------\n\nRectangle r3:");
  let mut r3 = Rectangle::create(30, 30);
  r3.print_area();
  println!("{}", r3);

  r3.width = 15;
  r3.print_area();
  println!("{}", r3);
}
