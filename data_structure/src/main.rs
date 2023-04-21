#![allow(unused)]

use std::vec;

fn main() {
    //tuples allow multiple data types to be stored
    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);
    println!("Name : {}", my_tuple.1);
    let(v1, v2, v3) = my_tuple;
    println!("Age: {}", v1);

    //String 
    let mut st1: String = String::new();
    st1.push('A');//first position in the string
    st1.push_str(" word");//second position in the string 
    for word in st1.split_whitespace(){
        println!("{}", word);
    }
    //replacing variables in old string and using new string
    let st2: String = st1.replace("A","Another");//Investigte issue of writing from and to manually
    println!("{}", st2);

    //Sort a string of characters
    let st3: String = String::from("x r t b h k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();//sorts 
    v1.dedup();//removes any duplicates that may arise
    for char in v1{
        println!("{}", char);
    }

    //Playing around with strings
    let st4: &str = "Random String";
    let mut st5: String = st4.to_string();
    println!("{}", st5);
    let byte_arr: &[u8] = st5.as_bytes();
    let st6: &str = &st5[0..6];//getting a slice of a string
    println!("String Length: {}", st6.len());
    st5.clear();//deleting values in a string if mutable

    //string by reference
    let st6: String = String::from("Just some");
    let st7: String = String::from(" words");
    let st8: String = st6 + &st7;//st6 will no longer exist as it is passed directly and not through inference like st7
    //let st8: String = &st6 + &st7; will bring an error because two &str cannot be added together
    println!("String : {}", st8); 

    //type casting
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);
    println!("{}", int3_u32);

    //enums
    enum Day{
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday, 
        Saturday,
        Sunday
    }

    impl Day{
        fn is_weekend(&self) -> bool{
            match self{
                Day::Saturday | Day::Sunday => true,
                _=> false 
            }
        }
    }
    let today:Day = Day::Monday;
    match today{
        Day::Monday => println!("Everyone Hates Tuesday"),
        Day::Tuesday => println!("Donut day"),
        Day::Wednesday => println!("Hump day"),
        Day::Thursday => println!("Pay day"),
        Day::Friday => println!("Almost weekend"),
        Day::Saturday => println!("Weekend"),
        Day::Sunday => println!("Weekend"), 
    }

    println!("Is today the weekend? {}", today.is_weekend());

    //vectors store values of the same type
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1,2,3,4];
    vec2.push(5);
    println!("1st : {}", vec2[0]); 
    let second = &vec2[1];//checking for the existence of values
    /*match vec2.get(index: i32 = 1){
        Some(second: &i32) => println!("2nd : {}", second);
        None => println!("No 2nd value");
    }
    for i: &mut i32 in &mut vec2{
        *i *= 2;
    }
    for i: &i32 in &vec2{
        println!("{}", i);
    }*/
    println!("Vec length {}", vec2.len())

}
