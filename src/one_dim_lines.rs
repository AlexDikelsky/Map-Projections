//num points between min and max, exclusive on upper end
// works with min>max, but goes backward
pub fn points_between_exclusive(min: f64, max: f64, num: usize) -> Vec<f64> {
    let inc = (max - min) / (num as f64);
    (0..num).map(|x| min + ((x as f64) * inc))
        .collect()
}

//num points between min and max, inclusive
pub fn points_between_inclusive(min: f64, max: f64, num: usize) -> Vec<f64> {
    let inc = (max - min) / ((num-1) as f64);
    (0..num).map(|x| min + ((x as f64) * inc))
        .collect()
}

// Exclude both ends of range
// useful for excluding extreme lines for tissot's indicatrix
pub fn points_between_exclusive_both_ends(min: f64, max: f64, num: usize) -> Vec<f64> {
    let inc = (max - min) / ((num+1) as f64);
    (0..num).map(|x| inc + min + ((x as f64) * inc))
        .collect()
}
