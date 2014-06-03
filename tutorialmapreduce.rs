extern crate collections;
use std::fmt::Show; 
use std::hash::Hash;
use collections::HashMap;

fn main() {
	
	// some strings with some words
	let mut strings: Vec<String> = vec!("these are words".to_string(), 
		"those are words".to_string(), 
		"lots of words".to_string());
	
	// function for map
	fn create_pairs(s: &String) -> Vec<(String, int)> {
		let mut retvals: Vec<(String,int)> = vec!(); 
		for word in s.as_slice().split(' ') {
			retvals.push((word.to_string(), 1));
		}
		retvals
	}
	
	// function for reduce
	fn reduce_pairs(key: String, vals: Vec<int>) -> Vec<(String, int)> {
		let mut total: int = 0;
		for val in vals.iter() {
			total += *val;
		}
		vec!((key, total))
	}
	
	// let's do it
	strings.mapreduce::<String,int>(create_pairs, reduce_pairs);	
}

trait MapReduce {
	fn mapreduce<K: Clone + Show + Hash + Equiv<K> + Eq, V: Clone + Show>(&mut self, fn(&String) -> Vec<(K, V)>, 
		fn(K, Vec<V>) -> Vec<(K, V)>);
}

impl MapReduce for Vec<String> {
	fn mapreduce<K: Clone + Show + Hash + Equiv<K> + Eq, V: Clone + Show>(&mut self, mapf: fn(&String) -> Vec<(K, V)>, 
		redf: fn(K, Vec<V>) -> Vec<(K, V)>) {
		
		let mut values_lists: Vec<Vec<(K, V)>> = vec!();
		
		// map 
		for item in (*self).iter() {
			values_lists.push(mapf(item)); 
		}
		
		// intermediate 
		let mut kv_map: HashMap<K, Vec<V>> = HashMap::new();
		for list in values_lists.iter() {
			for pair in list.iter() {
				let mut key: K;
				let mut val: V;
				match pair.clone() {
					(ref a, ref b) => {
						key = a.clone();
						val = b.clone();
					}
				}
				
				if kv_map.contains_key_equiv(&key) {
					kv_map.get_mut(&key).push(val);
				}
				else {
					kv_map.find_or_insert(key, vec!(val));
				}
			}		
		}
			
		// reduce
		for key in kv_map.keys() {
			let vals = kv_map.get(key);
			let rvals = redf(key.clone(), vals.clone());
			println!("{}", rvals);		
		}		
	}
}	
