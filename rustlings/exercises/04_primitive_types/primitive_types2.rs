// Characters (`char`)

fn main() {
    // シングルクオーテーションとダブルクオーテーションで囲まれた文字の型は違うものになります。
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("アルファベット順！");
    } else if my_first_initial.is_numeric() {
        println!("数値的！");
    } else {
        println!("アルファベットも数字もダメ！");
    }

    // TODO: 上記の例のように`your_character`変数を定義し、あなたの好きな文字を入れてください。
    // 文字(一文字)や特殊文字、異言語の文字や絵文字も試してみてください。
    // let your_character = '';

    let your_character = '%';
    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
