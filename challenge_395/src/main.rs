fn main() {

    let input1: [u32; 0] = [];
    let input2: [u32; 5] = [0, 0, 0, 0, 0];
    let input3: [u32; 5] = [1, 1, 1, 1, 1];
    let input4: [u32; 11] = [0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1];
    let input5: [u32; 11] = [1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0];
    let input6: [u32; 13] = [0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1];
    let input7: [u32; 15] = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];

    let inputs: [&[u32]; 7] = [&input1, &input2, &input3, &input4, &input5, &input6, &input7];

    for input in inputs {
        println!("{:?}", nonogramrow(input))
    }
}


fn nonogramrow(row: &[u32]) -> Vec<usize> {
    // initialize new vector
    let mut row_str: Vec<String> = Vec::new();
    // fill vector with ints converted to strings
    for bit in row {
        row_str.push(bit.to_string())
    }
    // join vector into complete strings
    let joined: String = row_str.join("");
    // split strings by 0 into vectors
    let split: Vec<&str> = joined.split("0").collect();
    // convert vector of references to vector of strings
    // let split_str: Vec<String> = split.iter().map(|&x| x.into()).collect();
    // create vector of non-zero lengths
    let vec: Vec<usize> = split
        .iter()
        .filter(|&x| x.len() != 0)
        .map(|&x| x.len())
        .collect();
    vec
}

