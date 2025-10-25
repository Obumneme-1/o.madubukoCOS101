use std::io;

fn main() {
    println!("Enter the value of a");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:f64 = input1.trim().parse().expect("Failed to input");

    println!("Enter the value of b");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f64 = input2.trim().parse().expect("Failed to input");

    println!("Enter the value of c");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:f64 = input3.trim().parse().expect("Failed to input");

    let disc = b * b - 4.0*a*c;

    if disc > 0.0 { println!("there are 2 real roots");
    let root1 = (-b) / (2.0*a) + disc.sqrt() / (2.0*a);
    let root2 = (-b) / (2.0*a) - disc.sqrt() / (2.0*a);
    println!(" the roots are : {} and {}", root1, root2);
    }
    else if disc == 0.0 {
    let root3 = -b / (2.0*a);
    println!( "the root of the equation is: {}", root3);
    }
    else if disc < 0.0 {
     println!("there are no real roots");
     let root4 =  (-b) / (2.0*a) + (-disc).sqrt() / (2.0*a);
     let root5 =  (-b) / (2.0*a) - (-disc).sqrt() / (2.0*a);
     println!( "the imaginary roots of the equation are {}i and {}i", root4, root5);
    }
    
}