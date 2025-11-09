use std::io;


fn main() {
    println!("Food Code       Menu                      Price");
    println!("P               Pound Yam/Edinkaiko      -N3,200");
    println!("F               Fried Rice & Chicken     -N3,000");
    println!("A               Amala & Ewedu Soup       -N2,500");
    println!("E               Eba & Egusi Soup         -N2,000");
    println!("w               White Rice & Stew        -N2,500");

println!("Enter the Food  code");

let mut f1 = String::new();
let mut f2 = String::new();  

io::stdin().read_line(&mut f1).expect("Invalid input");
let f1 = f1.trim().to_uppercase();

println!("Enter the quantity that you wish to order");
io::stdin().read_line(&mut f2).expect("Invalid input");
let f2:u32 = f2.trim().parse().expect("Enter an integer");

let a1 = 3200 * f2;
let a2 = 3200 * 95 / 100 *f2;
let b1 = 3000 * f2;
let b2 = 3000 * 95 / 100 * f2;
let c1 = 2500 * f2;
let c2 = 2500 * 95 / 100 * f2;
let d1 = 2000 * f2;
let d2 = 2000 * 95 /100 * f2;
let e1 = 2500 * f2;
let e1 = 2500 * 95 / 100 * f2;


if f1 == "P"{
        if a1 >= 5000 { println!("The total cost is {}", a2);
        } 

        else{println!("The value of a is {}", a1); }
        }
else  if f1 == "F"{

        if b1 >= 5000 { println!("The total cost is {}", b2);
        } 

        else{println!("The value of a is {}", b1);} 
        }
else if f1 == "A"{
        if a1 >= 5000 { println!("The total cost is {}", a2);
        } 

        else{println!("The value of a is {}", a1);}
        }
else if f1 == "E"{

        if a1 >= 5000 { println!("The total cost is {}", a2);
        } 

        else{println!("The value of a is {}", a1);}
        }
else  if f1 == "W"{

        if a1 >= 5000 { println!("The total cost is {}", a2);}
        } 

else {println!(" Couldn't read input for food code"); 
     }
}
