use add_one;
use add_two;
use add_three;

extern "C" {
	fn doubler(x: i32) -> i32;
	fn doublercpp(x: i32) -> i32;
}

fn main() {
    let num = 78;
    println!(
		"Hello, world! {} plus one is {} plus two is {} plus three is {}",
		num,
		add_one::add_one(num),
		add_two::add_two(num),
		add_three::add_three(num));

	unsafe {
		println!("calling native lib: {}", doubler(1));
		println!("calling native lib: {}", doublercpp(2));
	}
}
