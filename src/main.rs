use std::iter::Iterator;
use std::thread::sleep;
use std::time::Duration;
use numbers::*;

mod numbers;

fn main() {
    let mut minutes = 0;
    let mut seconds = 0;

    loop {
        let printables = [
            NUMBERS[minutes / 10], SPACE, NUMBERS[minutes % 10],
            COLON,
            NUMBERS[seconds / 10], SPACE, NUMBERS[seconds % 10]];
        print!("\x1b[1;1H\x1b[2J");
        for i in 0..11 {
            printables.iter().map(|&x| x[i]).for_each(|x| print!("{}", x));
            println!()
        }
        sleep(Duration::from_secs(1));
        seconds += 1;
        minutes += seconds / 60;
        seconds %= 60;
        minutes %= 60;
    }
}