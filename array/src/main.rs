#![allow(unused)]

fn main() {
    //printing value in a given array and its length
    let arr_1: [i32; 4] = [1,2,3,4];
    println!("1st: {}", arr_1[0]);
    println!("length: {}", arr_1.len());

    //program to print the even and odd numbers in an array
    let arr_2: [i32; 9] = [1,2,3,4,5,6,7,8,9];
    let mut loop_idx:usize = 0;
    //without the last line of the loop the loop will print 1 ad infinitum
   /* loop{
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_2[loop_idx] == 9 {
            break;
        }
        println!("Val : {}", arr_2[loop_idx]);
        loop_idx += 1;}*/ 
    
    //while loop
   /*while loop_idx < arr_2.len(){
        println!("Arr : {}", arr_2[loop_idx]);
        loop_idx += 1;
    } */

    //for loop  
    for val in arr_2.iter(){
        println!("Val : {}", val);
    }


}
