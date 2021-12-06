use std::fs::read_to_string;

fn main() {
    // expect() is similar to unwrap() but expect() gives context
    let contents = read_to_string("input.txt").expect("file should exist");
    /*
    unwrap() tells if something exists and takes that value that may or may not be there and then outputs it (crashes if not
    present)
        1. lines() gives an iterator of strings (for each line of the input file)
        2. taking the iterator from lines() and mapping to parse that string to an u32
            "map shoves another step between looking a prev value and next value" - Addison
        3. use unwrap() because we don't want to continue if something may fail
        4. collect() -> puts things into something else -> here we specified a vector
        5. windows will literally take a window of size two and slide along the vector
        6. filter then takes the two items from the window and does comparison -> if the comparison is True, filter adds that
        to return
        7. count will uh count each time the comparison is True
            - count tells you the number of elements in an iterator
     */
    let result = contents.lines().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>().windows(2).filter(|arr| {
        arr[0] < arr[1]
    }).count();

    println!("count: {}", result);
}


fn day_1_but_its_for() {
    let mut numbers = Vec::new();
    for line in read_to_string("input.txt").expect("file should exist").lines() {
        numbers.push(line.parse::<u32>().unwrap());
    }
    let mut count = 0;
    for idx in 1..numbers.len() {
        if numbers[idx-1] >= numbers[idx] {
            continue;
        }
        count += 1;

    }
    println!("count: {}", count);
}