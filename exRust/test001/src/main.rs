fn main() {
    let mut x = 1;
    {
        x = x + 1;
    }
    println!("x = {}\n",x);
}
