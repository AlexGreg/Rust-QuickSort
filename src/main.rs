use std::io;

fn main() {
    let mut array = [3,9,2,6,1,8,4,2,7];
    let len:uint = array.len();
    print!("array before sort: ");
    print_array(array);
    quicksort(array, 0, len-1);
    print!("array after sort: ");
    print_array(array);
}

fn partition(array:&mut[int], left:uint, right:uint) -> uint {
    let mut i:uint = left;
    let mut j:uint = right;
    let mut temp:int;
    let pivot:int = array[(left+right)/2u];

    while i <= j {
        while array[i] < pivot {
            i = i+1u;
        }
        while array[j] > pivot {
            j = j-1u;
        }
        if i <= j {
            temp = array[i];
            array[i] = array[j];
            array[j] = temp;
            i = i+1u;
            j = j-1u;
        }
    }

    i
}

fn quicksort(array:&mut[int], left:uint, right:uint) {
    let index:uint = partition(array, left, right);
    if left < index -1u {
        quicksort(array, left, index - 1u);
    }
    if index < right {
        quicksort(array, index, right);
    }
}

fn print_array(array:&[int]) {
    print!("[ ")
    for i in array.iter() {
        print!("{} ", i);
    }
    println!("]");
}