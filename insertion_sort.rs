use std::fmt::Display;

fn insertion_sort<T: PartialOrd>(array: &mut [T]) -> () {
	for mut i in 1..array.len() {
		let mut j = i - 1;
		while array[i] < array[j] {
			array.swap(i, j);
			if j >= 1 {
				j -= 1;
				i -= 1;
			} else {
				break;
			}
		}
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
	insertion_sort(&mut int_array);
	insertion_sort(&mut float_array);
	insertion_sort(&mut char_array);
	print(&int_array);
	print(&float_array);
	print(&char_array);
}

