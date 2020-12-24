use std::fmt::Display;

fn shell_sort<T: PartialOrd + Copy>(array: &mut [T]) -> () {
	let mut h = 1;
	while h < array.len() {
		h = (3 * h) + 1;
	}
	while h > 0 {
		for i in h..array.len() {
			let value = array[i];
			let mut j = i;
			while j > h -1 && value <= array[j - h] {
				array[j] = array[j - h];
				j = j - h;
			}
			array[j] = value;
		}
		h = h / 3;
	}
}

fn print<T: Display>(array: &[T]) -> () {
	for i in 0..array.len() {
		print!("{} ", array[i]);
	}
	println!("");
}

fn main() {
	let mut int_array = [5, 2, 1, 4, 3, 9, 6, 10, 8, 7];
	let mut float_array = [5.1, 2.2, 1.3, 4.4, 3.5, 10.6, 6.7, 9.8, 8.9, 7.1];
	let mut char_array = ['e', 'b', 'a', 'd', 'c', 'h', 'g', 'f', 'j', 'i'];
	shell_sort(&mut int_array);
	shell_sort(&mut float_array);
	shell_sort(&mut char_array);
	print(&int_array);
	print(&float_array);
	print(&char_array);
}
