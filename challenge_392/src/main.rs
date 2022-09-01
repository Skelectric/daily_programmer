#![allow(unused_variables)]
#![allow(dead_code)]

use rand::Rng;

fn main() {

    let vector1 = vec![0, 1, 2, 3, 4];
    let vector2 = vec![1, 2, 2, 2];
    let vector3 = make_vec_of_randints(1000, 10);

    let flipped1 = flipfront(&vector1, 2);
    assert_eq!(flipped1, vec![1, 0, 2, 3, 4]);

    let flipped2 = flipfront(&vector1, 3);
    assert_eq!(flipped2, vec![2, 1, 0, 3, 4]);

    let flipped3 = flipfront(&vector1, 5);
    assert_eq!(flipped3, vec![4, 3, 2, 1, 0]);

    let flipped4 = flipfront(&vector2, 3);
    assert_eq!(flipped4, vec![2, 2, 1, 2]);

    println!("Vector {:?} is_sorted: {}", vector3, is_sorted(&flipped3));

    let vector3 = pancake_sort(&vector3);

    println!("Vector {:?} is_sorted: {}", vector3, is_sorted(&vector3));

}


fn pancake_sort(vector: &Vec<usize>) -> Vec<usize>{
    let mut sorted = vector.clone();
    println!("Performing pancake sort on vector...");
    for i in 0..vector.len() {
        let slice: Vec<usize> = sorted[..sorted.len()-i].into_iter().copied().collect();
        let curr_max_index = position_max(&slice);
        // println!("curr_max_index = {}", curr_max_index);
        sorted = flipfront(&sorted, curr_max_index);
        // println!("first flipfront: {:?}", sorted);
        sorted = flipfront(&sorted, vector.len() - i);
        // println!("second flipfront: {:?}", sorted);
    }
    sorted
}


fn flipfront(vector: &Vec<usize>, n: usize) -> Vec<usize> {
    let mut rev_slice: Vec<_> = vector[0..n].into_iter().copied().rev().collect();  // split and reverse first slice
    let mut unrev_slice: Vec<_> = vector[n..].into_iter().copied().collect();  // instantiate second slice
    rev_slice.append(&mut unrev_slice);
    let rev_slice = rev_slice.clone();
    // print_type_of(&rev_slice);
    // println!("Flipping first {:?} elements of vector {:?} -> {:?}", n, vector, rev_slice);
    rev_slice
}

fn make_vec_of_randints(upper_val: usize, upper_len: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let vec: Vec<usize> = (0..upper_len).map(|_| rng.gen_range(0..upper_val)).collect();
    // print_type_of(&vec);
    vec
}

fn print_type_of<T: std::fmt::Debug>(var: &T) {
    println!{"type of {:?}: {:?}", var, std::any::type_name::<T>()}
}

fn is_sorted(vector: &Vec<usize>) -> bool{
    let max: usize = vector[0];
    let mut sorted: bool = true;
    for i in 0..vector.len() - 1 {
        if vector[i] > vector[i+1] {
            sorted = false;
            break;
        }
    }
    // println!("Vector {:?} is sorted: {}", vector, sorted);
    sorted
}

fn vector_max(vector: &Vec<usize>) -> usize {
    let max = vector.iter().fold(0, |a,b| a.max(*b));
    // println!("Max of {:?} is {:?}", vector, max);
    max
}

fn position_max(vector: &Vec<usize>) -> usize {
    // get rightmost position of maximum
    let max = vector_max(&vector);
    let max_index = vector.iter().rev().position(|&p| p == max);
    let max_index = match max_index {
        Some(max_index) => vector.len() - max_index,
        None => 0
    };
    // println!("{} is at position {:?}", max, max_index);
    max_index
}