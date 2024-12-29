fn main(){
    let x: usize = 6; //変数の方usize,C言語で言うunsigntint ,使用するアーキテクチャに合わせた最適なサイズが用意される．
    let y: f64 = 1.5;

    //let z:f64 = (x / y);  Rustは型違いの演算不可
    let z:f64 = (x as f64) / y; //型変換はas
    println!("value of z: {}",z);

    //----------------tap記法------
    let tup = (500,6.3,1);
    let (x, y, z) = tup;

    println!("The values of x: {},y: {},z: {}",x,y,z);
    //--------------------------
    let t:(i32, f64, u8) = (100,2.5,2);

    let a = t.0;
    let b = t.1;
    let c = t.2;

    println!("The values of a: {},b: {},c: {}",a,b,c);
}