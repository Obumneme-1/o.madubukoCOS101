fn main() {
    //while mut x = 0;
    let mut x = 0;
    loop{
        x+=1;
        println!("x={}",x);

        if x==5 {
           break;
        }
    }
}