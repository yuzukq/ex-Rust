fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: `cat`タプルを以下のprintln!が適切に動くように分解してください。
    //let /* your pattern here */ = cat;
    let  (name, age)= cat; //tap記法    

    println!("{name} is {age} years old");
}
