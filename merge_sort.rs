use std::fmt::Display;

fn merge_sort<T: PartialOrd + Copy>(array: &mut [T], start: usize, end: usize) -> () {
	if start < end {
		let middle = (start + end) / 2;
		merge_sort(array, start, middle);
		merge_sort(array, middle + 1, end);
		merge(array, start, middle, end);	
	}
}

fn merge<T: PartialOrd + Copy>(array: &mut[T], start: usize, middle: usize, end: usize) -> () {
	let mut i = start;
	let mut j = middle + 1;
	let mut array_aux = Vec::new();
	while i <= middle && j <= end {
	 	if array[i] < array[j] {
	 		array_aux.push(array[i]);
	 		i += 1;
	 	} else {
	 		array_aux.push(array[j]);
	 		j += 1;
	 	}
	}
	while i <= middle {
	 	array_aux.push(array[i]);
	 	i += 1;
	} 
	while j <= end {
		array_aux.push(array[j]);
		j += 1;
	}
	for k in start..end + 1 {
		array[k] = array_aux[k - start];
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
	merge_sort(&mut int_array, 0, 9);
	merge_sort(&mut float_array, 0, 9);
	merge_sort(&mut char_array, 0, 9);
	print(&int_array);
	print(&float_array);
	print(&char_array);
}
