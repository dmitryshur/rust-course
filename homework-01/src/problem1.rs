pub fn sum(slice: &[i32]) -> i32 {
    let sum = slice.iter().fold(0, |acc, num| acc + num);
    sum
}

pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();

    for value in vs.iter() {
        let value_count = &new_vec.iter().filter(|&num| num == value).count();
        if *value_count == 0 {
            new_vec.push(*value);
        }
        
    }
    new_vec
}

pub fn filter() {
    true;
}
