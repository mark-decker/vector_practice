//Given a list of integers, use a vector and 
//return the median (when sorted, the value in the middle position) and 
//mode (the value that occurs most often; a hash map will be helpful here) 
//of the list.
//mark decker Nov 8 2024
//
//
//write functions to implement quicksort
//first we need to partition
//put all values smaller to start and bigger to the right
//return usize as index must be > 0
fn partition_vec(v: &mut [i32]) -> usize {  //pass in mutable slice, return usize
    //find the length we are working with
    let len = v.len();
    //chose last element as pivot
    let pivot = v[len - 1];
    //initialize at start of slice
    let mut i = 0;

    for j in 0..len - 1 {
        if v[j] <= pivot {
            v.swap(i,j);  //swap the values
            i += 1;       //move to next i of slice
        }
    }

    v.swap(i, len-1);

    return i;  //return the index we are left with
}

fn main() {

    //create vector using vec! macro
    //
    let mut vec = vec![1, 6, 76, 98, 104, 3, 0, 3, 25, 4, 8, 19];

    //first we need to sort the vector
    //test partition function
    let k = partition_vec(&mut vec);
    println!("{k}");

    for v in &vec {
        println!("{v}");
    }

}
