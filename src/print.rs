pub fn run() {
  // Print to console
  println!("Hello from the print.rs file");

  // Basic Formatting
  println!("{} is from {}", "Luke", "ON");

  // Positional Arguments
  println!("{0} is from {1} and {0} likes to {2}", "Luke", "ON", "code");

  // Name Arguements
  println!(
    "{name} likes to {activity}",
    name = "Luke",
    activity = "code"
  );

  // Placeholder traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

  // Placeholde for debug trait
  println!("{:?}", (12, true, "hello"));

  // Basic math
  println!("10 + 10 = {}", 10 + 10);
}
