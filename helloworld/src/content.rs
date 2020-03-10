pub fn run() {
	let contents: [&str; 12] = [
		"print",
		"variable",
		"types",
		"strings",
		"tuples",
		"arrays",
		"vectors",
		"conditionals",
		"loops",
		"functions",
		"pointers",
		"structs"
	];
	for i in 1..13 {
		println!("{}. {}", i, contents[i-1]);
	}
	let mut conten: Vec<&str> = vec![
		"Strawberry",
		"Kitty",
		"Sandwich"
	];
	println!("{:?}", conten);
}