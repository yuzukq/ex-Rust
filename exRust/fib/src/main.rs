use std::io;

fn fib(n:u32) -> u32 {
    if n == 1 {
        1
    }
    else if n == 2 {
        1
    }
    else  {
        fib(n-1) + fib(n-2)
    }
}

fn main() {
    let mut number = String::new();
    println!("第何項のフィボナッチ数を求めますか？");

    io::stdin()
    .read_line(&mut number)
    .expect("行の読み取りに失敗しました");

    
    let number: u32 = number.trim().parse().expect("数字を入力してください。");

    println!("{number} 番目のフィボナッチ数は {} です",fib(number));
}
