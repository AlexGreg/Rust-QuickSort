extern crate time;

use std::io;
use std::rand;

fn main() {
    let mut size:uint;
    loop {
        print!("Enter the dimension of the array to sort: ");
        let input = io::stdin().read_line().ok().expect("Failed to read line");
        let input_num:Option<uint> = from_str(input.as_slice().trim());
        size =  match input_num {
            Some(size) => {
                size
            },
            None => {
                println!("Please enter a positive number!");
                continue;
            }
        };
        break;
    }
    let mut array = vec![];
    while array.len() < size {
        array.push(rand::random::<int>() % 1000i)
    }
    let len:uint = array.len();
    //print!("array before sort: ");
    //print_array(array.as_mut_slice());
    let start_time:time::Timespec = time::get_time();
    quicksort(array.as_mut_slice(), 0, len-1);
    let end_time:time::Timespec = time::get_time();
    //print!("array after sort: ");
    //print_array(array.as_mut_slice());
    println!("{}",end_time.sub(&start_time))
}

fn partition(array:&mut[int], left:uint, right:uint) -> uint {
    let mut i:uint = left;
    let mut j:uint = right;
    let mut temp:int;
    let pivot:int = array[(left+right)/2u];

    while i <= j {
        while array[i] < pivot {
            i = i+1u;
            if i == array.len()-1 {
                break;
            }
        }
        while array[j] > pivot {
            j = j-1u;
            if j == 0 {
                break;
            }
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