pub fn run() {
	let name = "Brad";
	let mut age = 37;

	println!("My name is {} and I am {}", name, age);

	age = 38;
	println!("My name is {} and I am {}", name, age);

	// Define consts
	const ID: i32 = 001;
	println!("ID: {}", ID);

	// Assign mult vars
	let (my_name, my_age) = ("Brad", 37);
	println!("{} is {}", my_name, my_age);
}