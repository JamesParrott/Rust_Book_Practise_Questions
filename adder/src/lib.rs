
//use rectangle_fits::Rectangle; 
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn exploration() {
        let result = 1.0 + 1.0 + 1.0 - 3.0;
        assert_eq!(result, 0.0);
    }
    // #[test]
    // fn float_representation() {
    //     let result = 0.1 + 0.1 + 0.1;
    //     assert_eq!(result, 0.3);
    // }
    // #[test]
    // fn panic() {
    //     panic!("Nope nope nope!");
    // }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}
