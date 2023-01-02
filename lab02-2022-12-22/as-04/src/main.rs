use std::io;
fn main() {
    let mut x:i32=0;
    let mut input = String::new();
    
    println!("Enter number: ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    x = input.trim().parse().expect("Not a valid number");

    print!("{} = ",x);
    
    let prime = [2, 3, 5, 7, 11];
    let mut result = [0; 50];

    for i in 0..100{
        for j in prime{
            if x%j == 0 {
                result[i] = j ;
                x = x/j;
                print!("{}",result[i]);
                if x != 1{
                    print!("*");
                }else{
                    break;
                }
                break;
            }
        }

    }

}