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
            //implement my own swap
            let tmp_val: i32 = v[j];  //don't borrow it!
            v[j] = v[i];
            v[i] = tmp_val;
            //v.swap(i,j);  //swap the values
            i += 1;       //move to next i of slice
        }
    }

    v.swap(i, len-1);

    return i;  //return the index we are left with
}

//quicksort function
//set the pivot with partition then call quick sort on two subslices
fn quick_sort(v: &mut [i32]) {  //don't return anything just call recursively

    //ensure we have a valid vector to know when to stop
    if v.len() <= 1 {
        return
    }

    let pivot = partition_vec(&mut v[..]);  //v is already a mutable reference here so no &mut 
                                            //unless I write it as a slice [..]

    quick_sort(&mut v[0..pivot]);  //start half
    quick_sort(&mut v[pivot + 1..]); //end half
}

fn main() {

    //create vector using vec! macro
    //
    let mut vec = vec![1, -6, 6, 76, 98, -75, 104, 3, 0, 3, 25, 4, 8, 19];

    //first we need to sort the vector
    //test partition function
    let k = partition_vec(&mut vec);
    println!("{k}");

    for v in &vec {
        println!("{v}");
    }

    quick_sort(&mut vec);

    println!("sould be sorted");
    for v in &vec {
        println!("{v}");
    }

    //now that is is sorted we just find value at middle index
    let len = vec.len() / 2;

    let median = &vec[len];

    println!("median is {median}");


}
