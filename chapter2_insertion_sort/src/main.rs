fn main() {

    let mut array_to_sort = [34, 15, 82, 46, 7, 14, 45, 64, 2, 99, 17, 97];
    print!("before sort: ");
    print_array(&mut array_to_sort);

    insert_sort(&mut array_to_sort);

    print!("\nafter sort: ");
    print_array(&mut array_to_sort);
}

fn insert_sort(array: &mut [i32]) { 
    // println!("in fn_insert_sort: ");
    
    'key_loop: for key_index in 1..array.len() {
        let mut index = key_index - 1;
        // println!("index in line 1: {index}");
        // println!("key_index in line 1: {key_index}");
        if array[key_index] > array[index] { continue; }

        let key = array[key_index];

        // end loop at the position where the key gets inserted
        while key < array[index] {
            // println!("inside comparing loop!");
            // println!("key: {key}"); 
            // println!("index's number in while loop: {}", array[index]);
            array[index + 1] = array[index]; 
            // println!("after move");
            // print_array(array);
            /*
            if index < 1 { 
                array[index] = key;
                // println!("after inserting key");
                // print_array(array);
                continue 'key_loop;
            } else { 
                index -= 1;
            };
            */
            if index > 0 {
                index -= 1;
            } else {
                array[index] = key;
                continue 'key_loop;
            }
        }

        // println!("target position: {}", index + 1);

        // adjust index and insert the key's value
        array[index + 1] = key;

        // println!("after inserting key");
        // print_array(array);

    }
}

fn print_array(array: &mut [i32]) {
    for item in &*array {
        print!("{item} ");
    }
    println!();
}
