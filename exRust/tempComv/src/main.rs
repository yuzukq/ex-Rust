use std::io;

fn celsius_to_fahrenheit(celsius:f64) -> f64{
    celsius * 1.8 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit:f64) -> f64{
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main(){
    println!("1: 華氏[F]から摂氏[C]\n2: 摂氏[C]から華氏[F]: \n");
    let mut choice = String::new();
    io::stdin()
    .read_line(&mut choice)
    .expect("行の読み取りに失敗しました");

    let choice: u32 = choice.trim().parse().expect("数字を入力してください");

    if choice == 1{
        println!("摂氏[C]を入力してください");
        let mut celsius = String::new();
        io::stdin()
        .read_line(&mut celsius)
        .expect("行の読み取りに失敗しました");

        let celsius: f64 = celsius.trim().parse().expect("数字を入力してください");
        println!("華氏は {}[F] です",celsius_to_fahrenheit(celsius));
    }
    else if choice == 2{
        println!("華氏[F]を入力してください");
        let mut fahrenheit = String::new();
        io::stdin()
        .read_line(&mut fahrenheit)
        .expect("行の読み取りに失敗しました");

        let fahrenheit: f64 = fahrenheit.trim().parse().expect("数字を入力してください");
        println!("摂氏は {}[C] です",fahrenheit_to_celsius(fahrenheit));
    }
    else{   
        println!("1か2を入力してください");
    }
}
    