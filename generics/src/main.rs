use std::ops::Add; //aids in the addition of generics

//function //allows addition of different data types

fn get_sum_gen<T:Add<Output = T>>(x:T, y:T)->T{
    return x + y;
}
fn main() {
    println!("5 + 4 = {}", get_sum_gen(5, 5));
    println!("5.2 + 4.6 =  {}", get_sum_gen(5.2, 4.6));
} 