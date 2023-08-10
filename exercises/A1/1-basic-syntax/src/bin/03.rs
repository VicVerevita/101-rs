fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    // TODO


    println!("{} is largest and {} is smallest", largest(&input), smallest(&input));
}

fn largest(array: &[i32]) -> i32 {
    let mut largest = 0;
    for i in 0..array.len() {
        if array[i] > largest {
            largest = array[i];
        }
    }

    return largest;
}

fn smallest(array: &[i32]) -> i32 {
    let mut smallest = array[0];
    for i in 0..array.len() {
        if array[i] < smallest {
            smallest = array[i];
        }
    }

    return smallest;
}