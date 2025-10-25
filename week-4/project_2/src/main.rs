use std::io;

fn main(){
    println!("Type the number '1' if the employee is experienced and the number '0' if he/she isn't");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).parse().expect("Failed to read input");

    println!("Enter the age(in numbers) of the employee");
    let mut age = i8::new();
    io::stdin().read_line(&mut age).parse().expect("Failed to read input");
    let a = 1_560_000;
    let b = 1_480_000;
    let c = 1_300_000;
    let d = 100_000;
    if input1 == 1 && age >= 40{println!("The employee's incentive is {}", a);}
    else if input1 == 1 && age >=30 {println!(" The employee's incentive is {}", b);}
    else if input1 == 1 && age >=28 {println!("The employee's incentive is {}", c);}
    else if input1 == 0 {println!(" The employee's pay is {}", d);}


}