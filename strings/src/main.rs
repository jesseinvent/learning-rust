fn main() {
    
    let mut s1 = String::from("tic");
    let mut s2 = String::from("tac");
    let mut s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("{}", s);
}
