fn fff() {
    let mut counter = 0;
    loop {
        if (counter >= 10) {
            break;
        }
        counter += 1;
    }

    while counter < 10 {
        counter += 1;
    }

    for counter in 0..11 {
        println!("{}", counter);
    }

    for counter in 0..11 {
        println!("{}", counter)
    }

    let array: [i32; 5] = [10, 11, 12, 13, 14];
    for index in 0..5 {
        println!("{}", array[index]);
    }
    for value in array.iter() {
        println!("{}", value);
    }
}
