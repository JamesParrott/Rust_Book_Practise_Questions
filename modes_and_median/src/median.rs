pub fn get_median(data: &Vec<isize>) -> f32 {
    let mut sorted = data.clone();
    sorted.sort_unstable();
    let len = sorted.len();


    if len == 1 {
        sorted[0] as f32
    } else if len % 2 == 1{
        sorted[len / 2] as f32   
    } else {
        let index = len / 2;
        let below = sorted[index - 1] as f32;
        let above = sorted[index] as f32;
        0.5 * (below + above)
    }

}