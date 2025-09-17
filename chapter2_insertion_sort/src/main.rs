fn main() {

    let mut array_to_sort = [4, 5, 2, 6, 7, 14, 45, 64, 2, 9, 17, 97];
    print!("before sort: ");
    print_array(&mut array_to_sort);

    insert_sort(&mut array_to_sort);

    print!("\nafter sort: ");
    print_array(&mut array_to_sort);
}

fn insert_sort(array: &mut [i32]) -> () { 
    print!("in fn_insert_sort: ");

    
    for item in 0..array.len() {
        let key = item;
        print!("{key} ");
        /*
        if key < array[array_position - 1] {
            array[array_position] = array[array_position - 1];
            array[array_position - 1] = key;
        }
        */
    }
}

fn print_array(array: &mut [i32]) -> () {
    for item in &*array {
        print!("{item} ");
    }

    println!();
}
