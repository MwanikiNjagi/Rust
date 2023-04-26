#[allow(unused)]

fn main() {
    struct Customer{
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob: Customer = Customer{
        name: String::from("Bob Smith"), 
        address: String::from("555 Main St"),
        balance: 234.50
    };
    //changing the contents of the structs
    bob.address = String::from("505 Main St");

    //struct and traits
    struct Rectangle{length: f32, width:f32};
    //take width as base
    struct Triangle{length: f32, width: f32};

    trait  Shape {
        fn new(length: f32, width:f32) -> Self;
        fn area(&self) -> f32;
    }

    impl Shape for Rectangle{
        fn new(length: f32, width:f32) -> Self {
            return Rectangle{length, width};
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        } 
        
    }

    impl Shape for Triangle{
        fn new(length: f32, width:f32) -> Self {
            return Triangle{length, width};
        }
        fn area(&self) -> f32 {
            return (self.width/2.0) * self.length;
        }
    }

    let rec: Rectangle = Shape::new(10.0,10.0);
    let tri: Triangle = Shape::new(10.0, 10.0);

    println!("Rec area: {}", rec.area());
    println!("Tri area: {}", tri.area());

}
