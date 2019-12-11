struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let input : Vec<u64> = vec![0, 1, 3, 6, 10];
    let output : Vec<u64> = parts_sums(&input);
    println!("Hello, world!");
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn parts_sums(ls: &[u64]) -> Vec<u64> {
    let mut vec : Vec<u64> = vec![0; ls.len() + 1];
    for i in 0..ls.len() {
        vec[0] += ls[i];
    }
    
    for i in 1..ls.len() {
        vec[i] = vec[i - 1] - ls[i - 1];
    }
    
    return vec;
}