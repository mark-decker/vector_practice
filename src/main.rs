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


//function to convert vec to a hash map
//keys with be the values in the vector
//count up each time we find that key
use std::collections::HashMap;  //bring hashmap into scope

fn hashmap_from_vec(v: &mut [i32]) -> HashMap <&mut i32, i32> {  //v does not have to be mutable
                                                                 //but it is
    let mut map = HashMap::new();
    //loop over vector adding
    for i in v {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    return map  //return the map

}

fn main() {

    //create vector using vec! macro
    //
    let mut vec = vec![1, -6, 6, 76, 98, -75, 104, 3, 0, 3, 25, 4, 8, 19, -6, 6, 1, 1, 1];
    //output vec
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

    //make a hashmap to count occurances
    let map = hashmap_from_vec(&mut vec);

    println!("{map:?}");

    //find the max value in the hashmap
    let mut max_count = 0; 
    for (key, count) in &map {  //use reference
        if *count > max_count {  //must deref
            max_count = *count;
        }
    }
    println!("max count is {max_count}");
    //print out key, count for max_count
    for (key, count) in &map {  //ref to map
        if *count == max_count {  //must deref due to using r3ef to map
            println!("The mode is {key} with a count of {max_count}");
        }
    }

}
