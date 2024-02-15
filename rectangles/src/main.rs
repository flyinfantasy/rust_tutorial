#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }


}

fn main() {
    let rect1 = Rectangle {
      width:30,
      height:50,  
    };

    let rect2: Rectangle = Rectangle { 
        width: 10, 
        height: 40, 
    };

    let rect3: Rectangle = Rectangle { 
        width: 60, 
        height: 45, 
    };

    println!("rect1 能容納 rect2 嗎？{}", rect1.can_hold(&rect2));
    println!("rect1 能容納 rect3 嗎？{}", rect1.can_hold(&rect3));

    println!(
        "長方形的寬度不為零，而是 {}",
        rect1.width
    );

    println!(
        "長方形的面積為 {} 平方像素。",
        rect1.area()
    );
}