#![allow(unused)]

fn get_sum(x:i32, y:i32){
    println!("{} + {} = {}", x, y, x+y);
}

//return type is indicated after the closing bracket of the function
fn get_sum2(x:i32, y:i32) -> i32{
    x+y

}

//return statement also necessitates semi colon at the end of the statement
fn get_sum3(x:i32, y:i32) -> i32{
    return x+y;

}

fn get_sum4(x:i32) -> (i32, i32){
    return(x+1, x+2);
}

fn sum_list(list: &[i32])->i32{
    let mut sum = 0;
    for &val in list.iter(){
        sum += &val;
    }
    sum
}

fn main() {
    get_sum(5,4);
    println!("{}", get_sum2(2,3));
    println!("{}", get_sum3(1, 5));
    
    //for get_sum4 
    let(val_1, val_2) = get_sum4(3);
    println!("Nums: {}, {}", val_1, val_2);
    
    //sum of a vec
    let num_list: Vec<i32> = vec![1,2,3,4,5];
    println!("Sum of a list = {}", sum_list(&num_list));

}
