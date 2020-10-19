struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: Rectangle) -> u32 {
    rect.width * rect.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, cmp: &Rectangle) -> bool {
        self.width >= cmp.width && self.height >= cmp.height
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    println!("Rectangle area is {}", area(Rectangle{height: 32, width: 50}));
    println!("Rectangle area is {}", Rectangle{height: 32, width: 50}.area());
    println!("Rectangle 1 can hold 2 {}", Rectangle{height: 2, width: 2}.can_hold(&Rectangle{height: 1, width: 1}));
    println!("{}", value(&Coin::Quarter));
}
