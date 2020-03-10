// Resizable arrays

pub fn run() {
	let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

	println!("{:?}", numbers);

	// Get single value
	println!("Single Value: {}", numbers[0]);

	// Reassign value
	numbers[2] = 20;

	println!("{:?}", numbers);

	// Add to vector
	numbers.push(5);
	numbers.push(6);
	println!("{:?}", numbers);

	// Pop off last value
	numbers.pop();
	println!("{:?}", numbers);

	// Get vector len
	println!("Vector length: {}", numbers.len());

	// Vectors are stack allocated
	println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

	// Get slice
	let slice: &[i32] = &numbers[0..2];
	println!("Slice: {:?}", slice);

	// Loop through vector values
	for x in numbers.iter() {
		println!("Number: {}", x);
	}

	// Loop & mutate values
	for x in numbers.iter_mut() {
		*x *= 2;
	}
	println!("{:?}", numbers);
}