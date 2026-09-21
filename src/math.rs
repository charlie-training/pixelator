use std::panic;

pub fn rbg_mean(ints: &Vec<u32>) -> u32 {
    if ints.len() == 0 {
        panic!("No values for average found!")
    }
    let av = ints.iter().sum::<u32>() / ints.len() as u32;
    av
}
