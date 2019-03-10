fn main() {
    // let mut counter = 0;
    // while counter < 10 {
    //     println!("{}", counter);
    //     counter += 1;
    // }

    // let mut counter = 0;
    // loop {
    //     if counter > 10 {
    //         break;
    //     }
    //     println!("{}", counter);
    //     counter += 1;
    // }

    let mut num_of_stars = 1;
    let num_of_rows = 9;

    for row in 0..num_of_rows {

        for _star in 0..num_of_stars {
            print!("*");
        }

        print!("\n");

        if row < (num_of_rows / 2) {
            num_of_stars += 1;
        } else {
            num_of_stars -= 1;
        }

    }

}