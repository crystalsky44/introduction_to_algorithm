fn main() {

    let mut array_to_sort = [34, 15, 82, 46, 7, 14, 45, 64, 2, 99, 17, 97];
    print!("before sort: ");
    print_array(&mut array_to_sort);

    insert_sort(&mut array_to_sort);

    print!("\nafter sort: ");
    print_array(&mut array_to_sort);
}

fn insert_sort(array: &mut [i32]) { 
    println!("in fn_insert_sort: ");
    
    for mut index in 0..array.len() {
        let key_index = index + 1;
        println!("index in line 1: {index}");
        println!("key_index in line 1: {key_index}");

        if key_index > 11 {
            break;
        } else if array[key_index] > array[index] {
            continue;
        }

        // end loop at the position where the key gets inserted
        while array[key_index] < array[index] {
            println!("inside comparing loop!");
            println!("key_index's number in while loop: {}", array[key_index]);
            println!("index's number in while loop: {}", array[index]);
            index = if index < 1 { break } else { index - 1 };
        }

        // adjust index
        index = if index > 0 { index + 1 } else { index };

        println!("target position: {index}");

        let key = array[key_index];
        for shift_step in (index..=key_index).rev() {
            println!("{shift_step}");
            if shift_step > 0 { 
                array[shift_step] = array[shift_step - 1];
                println!("in shift_step");
                print_array(array);
            }        
        }
        array[index] = key;
        println!("after move");
        print_array(array);

        /*
        for shift_step in 0..key_index {

        }
        */
    }
}

fn print_array(array: &mut [i32]) {
    for item in &*array {
        print!("{item} ");
    }

    println!();
}
