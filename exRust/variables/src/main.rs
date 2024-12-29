fn main(){
    let mut x: i32 = 5; //letで型宣言，:の後は記述しない場合コンパイル時に型が振られる
    const CONSTANT: usize = 100; //constで宣言された変数はコンパイル時に型が明示されている必要がある．

    println!("The value of CONSTANT is : {}",CONSTANT);
    println!("The value of x is : {}",x);

    x = 6;
    println!("The value of x is : {}",x);

    let some_strings: &str = "hoge";
    println!("The value of spaces is: {}", some_strings);

    //some_strings = some_strings.len();  再代入の場合，型が同じ必要がある．  
    let some_strings = some_strings.len(); //再宣言　の場合は型が変わっても大丈夫
    println!("The value of spaces is: {}", some_strings);


}