use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("./sample").unwrap();
    for line in contents.lines(){
        let mut it = line.split(' ');
        let first = it.next().unwrap();
        let second = it.next().unwrap().parse::<u64>().unwrap();
        println!("{} {}", first, second)
    }
}
