use std::fmt::Show; 

fn main() {
	let mut words: ~[String] = ~["these".to_string(), 
		"are".to_string(), "words".to_string()];
	
	fn create_pair(s: &String) -> (String, int) {
		(s.clone(), 1)
	}
	
	words.mapr(create_pair);	
}

trait Map {
	fn mapr<K: Show, V: Show>(&mut self, fn(&String) -> (K, V));
}

impl Map for ~[String] {
	fn mapr<K: Show, V: Show>(&mut self, f: fn(&String) -> (K,V)) {
		for item in (*self).iter() {
			println!("{}", f(item)); 
		}
	}
}	
