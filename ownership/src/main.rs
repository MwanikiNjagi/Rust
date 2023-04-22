#[allow(unused)]
//stack: Stores values in a last in first out format
//Data on the stack must have a defined fixed size

//Heap: When putting data on the heap you request a certain amount of space. The OS finds space available and returns an address for that space called a pointer
//Rules::
    //1. Each value has a variable that's called its owner
    //2. There is only one owner at a time
    //3. When the owner goes out of scope the value disappears

fn print_str(x: String){
    print!("A string {}", x);
}

fn print_return_str(x:String)-> String{
    println!("A string {}", x);
    x
}

fn change_string(name: &mut String){
    name.push_str("is happy");
    println!("Message : {}", name);
}

fn main() {
    let str1 = String::from("World");
    //let str2= str1;
    //println!("Hello {}", str1); will not work because str2 already replaced it
    let _str2 = str1.clone();
    println!("Hello {}", str1);//NB:will work with strings and vectors

    //print_str(str1);
    let _str3 = print_return_str(str1);
    println!("str3 = {}", _str3);

    //Changing string
    let mut str4 = String::from("Mwaniki");
    change_string(&mut str4);

}
