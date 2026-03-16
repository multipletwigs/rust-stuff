fn main() {
    let mut count = 0;

    'counting_up: loop {
        println!("Counting up until 2, at {count} now");
        let mut remaining = 10;

        loop {
            println!("Counting down to 0, at {remaining} now");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}")
}
