use std::fs;

fn main() {
    // Ex 1

    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8];

    let mut flag = 0;
    let mut i = 0;
    let mut j = 0;

    while i < org_arr.len() && j < sub_arr.len() {
        if org_arr[i] == sub_arr[j] {
            i += 1;
            j += 1;
            if j == sub_arr.len() {
                flag = 1;
            }
        } else {
            i = i - j + 1;
            j = 0;
        }
    }
    println!("{}", flag == 1);

    // Ex 2
    let t = "data";
    let filename = "src/1-s2.0-S0960982203005347-mmc6.txt";

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    
    let c = contents.matches(t).count();

    println!("{} occurrences", c);
}

