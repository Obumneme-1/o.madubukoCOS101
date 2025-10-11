fn main () {
let p:f64 = 510_000.0;
let r:f64 = 5.0;
let n:f64 = 3.0 ;
// amount after 3 years
let a = p*(1.0 -(r/100.0)).powf(n);
print!("The amount after 3 years is {}", a);
}