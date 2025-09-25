fn find_num_occurrence_array(n: u8, arr: [u8; 10]) -> usize {
    // TODO: find and return the number of occurrences of "n" in array "arr".
    let mut counter = 0;

    for i in 0..arr.len() {
        if arr[i] == n {
            counter += 1;
        }
    }
    println!("There are {counter} occurences of {n}");
    return counter;
}

fn find_num_occurrence_array_ref(n: u8, arr_ref: &[u8; 10]) -> usize {
    // TODO: find and return the number of occurrences of "n" in array referenced by "arr_ref".

    let mut counter = 0;

    for i in 0..arr_ref.len() {
        if arr_ref[i] == n {
            counter += 1;
        }
    }
    println!("There are {counter} occurences of {n}");
    return counter;
}

fn find_num_occurrence_slice(n: u8, slice: &[u8]) -> usize {
    // TODO: find and return the number of occurrences of "n" in slice reference "slice".
    let mut counter = 0;

    for i in 0..slice.len() {
        if slice[i] == n {
            counter += 1;
        }
    }
    println!("There are {counter} occurences of {n}");
    return counter;
}
// ------------------------------2.3.2--------------------------------------

fn increment_array (mut arr : [u8; 10]) -> [u8; 10] {
    // TODO: add one to every element of the array "arr", then return the array.
  }
  
  fn increment_array_ref (arr_ref : &mut [u8; 10]) {
    // TODO: add one to every element of the array "arr".  Nothing to return.
  }
  
  fn increment_slice (slice : &mut [u8]) {
    // TODO: add one to every element of the array "arr".  Nothing to return.
  }


fn main() {
    let array = [4, 5, 6, 7, 8, 9, 5, 5, 6, 10];
    // TODO: call find_num_occurrence_array in a loop (with every from 0 to 9 inclusive).
    for i in 0..10 {
        find_num_occurrence_array(i, array);
    }
    println!("------------------------------");
    // TODO: call find_num_occurrence_array_ref in a loop (with every from 0 to 9 inclusive).
    for i in 0..10 {
        find_num_occurrence_array_ref(i, &array);
    }
    
    println!("------------------------------");
    // TODO: call find_num_occurrence_slice in a loop (with every from 0 to 9 inclusive).

    for i in 0..10 {
        find_num_occurrence_slice(i, &array[6..10]);
    }

    // ------------------------2.3.2---------------------------------------------------



    let mut array : [u8; 10] = [4,5,6,7,8,9,5,5,6,10];
    dbg! (array);
    dbg! (increment_array (array));
    dbg! (array);
    dbg! (increment_array_ref (&mut array));
    dbg! (array);
    dbg! (increment_slice (&mut array));
    dbg! (array);
}
