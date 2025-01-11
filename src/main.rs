use std::fs;
use std::os::raw::c_void;

// helper functions //
fn read_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    return contents;
}

fn string_to_array(string: String) -> Vec<i64> {
    let numbers = string.split(" ").map(|number| number.parse::<i64>().unwrap()).collect();

    return numbers;
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

// quicksort functions
fn partition(vec: Vec<i64>, low: i64, high:i64) -> i64 {
    let pivot: i64 = high;
    let mut i = low;

    for j in vec[..=high as usize] {
        if vec[*j as usize] < pivot {
            i = i + 1;
            swap(vec, i, *j);
        }
    }

    print_type_of(&vec);
    return low + 1;
}

fn swap(vec: Vec<i64>, i:i64, j:i64) {
    let temp:i64 = vec[i as usize];
    vec[i as usize] = vec[j as usize];
    vec[j as usize] = temp;
}

fn quicksort(vec: Vec<i64>, low: i64, high: i64) {
    if low < high {
        let partition:i64 = partition(vec, low, high);

        quicksort(vec, low, partition-1);
        quicksort(vec, partition+1, high);
    }
}



fn main() {
    let string:String = read_file("data/100.txt");
    let mut array:Vec<i64> = string_to_array(string);
    println!("Here is the numbers inside the file: {:?}", array);

    quicksort(array, 0, (array.len() as i64));

    println!("Here is the numbers of file sorted: {:?}", array);
}
