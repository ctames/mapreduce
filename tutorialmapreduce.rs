use std::fmt::Show; 

fn main() {
	
	let mut strings: Vec<String> = vec!("these are words".to_string(), 
		"those are words".to_string(), 
		"lots of words".to_string());
	
	// function for map
	fn create_pairs(s: &String) -> Vec<(String, int)> {
		let mut retvals: Vec<(String,int)> = vec!(); 
		for word in s.as_slice().split(' ') {
			println!("{}", word);
			retvals.push((word.to_string(), 1));
		}
		retvals
	}
	
	// function for reduce
	fn reduce_pairs(pairs: Vec<(String, int)>) -> Vec<(String, int)> {
		vec!(("".to_string(), 1))
	}
	
	// let's do it
	strings.mapreduce::<String,int>(create_pairs, reduce_pairs);	
}

trait MapReduce {
	fn mapreduce<K: Show, V: Show>(&mut self, fn(&String) -> Vec<(K, V)>, 
		fn(Vec<(K, V)>) -> Vec<(K, V)>);
}

impl MapReduce for Vec<String> {
	fn mapreduce<K: Show, V: Show>(&mut self, mapf: fn(&String) -> Vec<(K, V)>, 
		redf: fn(Vec<(K, V)>) -> Vec<(K, V)>) {
		
		let mut values: Vec<(K, V)> = vec!();
		for item in (*self).iter() {
			values = mapf(item);
			for pair in values.iter() {
				println!("{}", pair); 
			} 
		}
		
		// then reduce
	}
}	
