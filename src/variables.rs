pub fn run() {
  let name = "Luke";
  let mut age = 22;
  println!("My name is {} and I am {}", name, age);

  age = 23;

  println!("My name is {} and I am {}", name, age);

  // Define constant
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // Assugn multiple vars
  let (my_name, my_age) = ("Luke", 22);
  println!("{} is {}", my_name, my_age);
}
