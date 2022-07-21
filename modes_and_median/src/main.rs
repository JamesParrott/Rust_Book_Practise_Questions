mod modes;
mod median;

use get_args;//::{get_isizes, print_vec_of_ints};
use modes::get_modes;
use median::get_median;

fn main() {
    let data = get_args::get_isizes();
    get_args::print_vec_of_ints(&data);
    println!("",);
    if data.len() >= 1 {
        let modes = get_modes(&data);
        let median = get_median(&data);
        println!("Mode(s) == {:?}", modes);
        println!("Median == {median}");
    }
}
