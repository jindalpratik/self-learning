use std::collections::HashMap;

fn main() {
    let mut v = vec![5, 4, 8, 3, 6, 9, 2, 4, 7, 5, 6, 8, 9];
    println!("The median value of vector is {}.", median_of_vector(v.clone()));
    println!("The mode of vector is {}", mode_of_vector(&v));
    v.push(5);
    v.push(3);
    v.push(3);
    println!("The new median of the vector is {}.", median_of_vector(v.clone()));
}

fn median_of_vector(mut v: Vec<i32>) -> i32 {
    v.sort();
    v[v.len()/2]
}

fn mode_of_vector(v: &Vec<i32>) -> i32 {
    
    let mut map = HashMap::new();
    for i in v {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut mod_key = 0;
    let mut mod_value = 0;

    for (key, value) in map {
        if value >= mod_value {
            mod_value = value;
            if *key > mod_key {
                mod_key = *key;
            }
        }
    }
    mod_key
}