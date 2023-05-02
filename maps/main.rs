#[allow(unused)]
fn main() {
    let mut arr_it = [1,2,3,4];
    for val in arr_it.iter(){
        println!("{}", val);
    }

    //this is an iterator
    let mut iter1 = arr_it.iter();
    println!("1st : {:?}", iter1.next());

    //closure- a function without a name 
    //let var_name = |parameters|-> return_type {BODY}
    let can_vote = |age: i32|{
        age >=18
    };
    println!("Can vote : {}", can_vote(8));

    //closures can access variables outside of their body
    let mut samp1 = 5;
    let print_var = || println!("samp1 = {}", samp1);
    print_var();
    samp1 = 10;
    let mut change_var = || samp1 += 1;
    change_var();
    println!("samp1 = {}",samp1);
    samp1 = 10;
    println!("samp1 = {}",samp1);

    //passing closures into functions
    //you can create a function inside main
    fn use_func<T>(a:i32, b:i32, func: T)-> i32
    where T:Fn(i32, i32) -> i32{
        func(a, b)
    }
    //above function takes closures and then performs operations
    let sum = |a,b| a+b;
    let prod = |a,b| a*b;
    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("5 * 4 = {}", use_func(5, 4, prod));

}
