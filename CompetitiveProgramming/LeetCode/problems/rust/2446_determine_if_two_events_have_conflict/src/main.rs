fn main() {
    let event1 = vec![String::from("15:19"), String::from("17:56")];
    let event2 = vec![String::from("14:11"), String::from("20:02")];
    println!("{}", have_conflict(event1, event2));
}

pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
    ((event2[0] >= event1[0] && event2[0] <= event1[1])
        || (event2[1] >= event1[0] && event2[1] <= event1[1]))
        || ((event1[0] >= event2[0] && event1[0] <= event2[1])
            || (event1[1] >= event2[0] && event1[1] <= event2[1]))
}

// Optimal Solution
pub fn have_conflict_optimal(event1: Vec<String>, event2: Vec<String>) -> bool {
    event1[0] <= event2[1] && event1[1] >= event2[0]
}
