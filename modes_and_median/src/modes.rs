use std::collections::HashMap;
use std::cmp::max;

pub fn get_modes(data: &Vec<isize>) -> Vec<isize> {

    let mut freqs = HashMap::new();
    let mut modes = Vec::new();
    let mut max_freq: u16 = 0;

    for x in data{
        let freq = freqs.entry(x).or_insert(0);
        *freq += 1;
        max_freq = max(max_freq, *freq);
    }

    for (x, freq) in freqs {
        if freq == max_freq{
            modes.push(*x);
        }
    }
    modes
}