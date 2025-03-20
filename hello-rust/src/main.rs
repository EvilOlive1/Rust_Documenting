#[allow(unused_variables)]

fn main()
{
	// Printing a line
    println!("Hello, world!");

	// '_' before a varaible name suppresses 'unused variable'
	let x: i32 = 5;

	// 'mut' Keyword
	let mut x: i32 = 1;
	x = 7;
	let mut x = x;
	x = x + 3;
	
	// Asserting Equalities
	let a: i32 = 5;
	assert_eq!(a, 5);

	// Use a pattern with 'let' to destructure a tuple to sep variables
	let (mut a, b) = (1, 2);
	a += 2;
	assert_eq!(a, 3);
	assert_eq!(b, 2);

	// Use tuple, slice, and struct patterns as the left side of an assignment
	let (c, d);
	(c, ..) = (3, 4);
	[.., d] = [1, 2];
	assert_eq!([c, d], [3, 2]);
	
					
}

fn _define_x()
{
	let x: &str = "hello";
	println!("{}, world", x);
}
