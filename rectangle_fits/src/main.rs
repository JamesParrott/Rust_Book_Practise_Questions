#[derive(Debug)]
pub struct Rectangle {
    width : usize,
    height : usize,
}

impl Rectangle{
    fn can_hold(&self, rect : &Rectangle) -> bool {
        let fits_parallel: bool = self.width >= rect.width && self.height >= rect.height;
        let fits_perpendicular: bool = self.width >= rect.height && self.height >= rect.width;
        fits_parallel || fits_perpendicular // ignore diagonal
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}