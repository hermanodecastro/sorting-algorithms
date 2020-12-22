use std::fmt::Display;

fn quick_sort<T: PartialOrd>(array: &mut [T], start: usize, end: usize) -> () {
	if array.len() > 1 {
		let pivot = (start + end) / 2;
		let mut left = start;
		let mut right = end;
		while left <= right {
			while array[left] < array[pivot] {
				left += 1;
			}
			while array[right] > array[pivot] {
				right -= 1;
			}
			if left <= right {
				array.swap(left, right);
				left += 1;
				right -= 1;
			}
		}
		if start < right {
			quick_sort(array, start, right);	
		}
		if end > left {
			quick_sort(array, left, end);
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
	quick_sort(&mut int_array, 0, 9);
	quick_sort(&mut float_array, 0, 9);
	quick_sort(&mut char_array, 0, 9);
	print(&int_array);
	print(&float_array);
	print(&char_array);
}