use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");//配列外の値にアクセスるとRustの安全機構で"パニック"
           // 配列の何番目の要素にアクセスするか指定してください

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
              // 値の読み込みに失敗しました

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
        // 入力された値は数字ではありません

    let element = a[index];

    println!(
        "{} 番目の配列の要素は {}",
        // {}番目の要素の値は{}です
        index, element
    );
}