pub fn run() {
	// Primitive str
	let strr = "hello";

	// Growable str
	let mut str2 = String::from("hello ");

	println!("{}", strr);

	// Get length
	println!("Length: {}", strr.len());

	// Push char
	str2.push('w');
	println!("{}", str2);

	// Push string
	str2.push_str("orld!");
	println!("{}", str2);

	// Loop through str by whitespace
	for word in str2.split_whitespace() {
		println!("{}", word);
	}

	// Create str with capacity
	println!("Create str with capacity");
	let mut s = String::with_capacity(10);
	s.push('a');
	s.push('b');

	println!("{}", s);

	// Assertion testing
	assert_eq!(3, s.len());


}