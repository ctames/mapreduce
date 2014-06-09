// generics.rs

fn fill_a_vector<T: Clone + Copy>(param: &T) -> Vec<T> {
	let mut a_vec: Vec<T> = Vec::new();
	let the_param = param.clone();
	for _ in range(0,5) {
		a_vec.push(the_param); 
	}
	a_vec
}

fn main() { 
	let a_vec: Vec<int> = fill_a_vector::<int>(&1);
	println!("{}", a_vec);
}