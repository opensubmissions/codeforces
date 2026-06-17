use std::io;
fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let w: i32 = s.trim().parse().unwrap();
    if w > 2 && w % 2 == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}