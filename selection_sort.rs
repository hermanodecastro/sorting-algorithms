use std::fmt::Display;

fn selection_sort<T: PartialOrd>(array: &mut [T]) -> () {
	for i in 0..array.len() {
		let mut smaller = i;
		for j in i + 1..array.len() {
			if array[j] < array[smaller] {
				smaller = j;
			}
		}
		array.swap(i, smaller);
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
	selection_sort(&mut int_array);
	selection_sort(&mut float_array);
	selection_sort(&mut char_array);
	print(&int_array);
	print(&float_array);
	print(&char_array);
}
