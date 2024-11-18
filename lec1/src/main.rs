fn main() {
    // println!("Hello, world!");
    // let mut num:u8 = 22;
    // let num =58;
    // println!("This is stored in num {}",num);
    // num=100;
    // println!("This is stored in num {}",num);

    // String-Dynamic length strings - heap allocated
     // &str - special memory fixed string
    // let mut  sentence:String = String::from("this is code editor");
    // sentence.push_str(" Done");
    // print!("{}",sentence)

    // tuple
    // let emp_info:(&str,u8)= ("Ramesh",50);
    // println!("{}",emp_info.0);
    // println!("{}",emp_info.1);

    // let (emp_name,emp_age)=emp_info;
    // println!("{} and {}",emp_name,emp_age);


    // print_value(5);
    // let ans:u8=add(5,6);
    // println!("{}",ans);

    let str1=String::from("hello");
    let str2=str1;
    // println!("{}",str1);
    println!("{}",str2);
}

// fn print_value(num:u8){
//     println!("This is print value funtion");
//     println!("{}",num);
// }
// fn add(num:u8,num2:u8) -> u8{
//     return num + num2 ;
// }
