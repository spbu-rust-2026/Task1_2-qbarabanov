use std::io;

fn main() {
    let mut total: i128 = 0;
    let mut flag: bool = false;

    loop {
        let result = read();

        if result.0 == -1 {
            break;
        }

        total += if result.0 > 0 {
            flag = result.1;
            result.0
        } else {
            flag = true;
            0
        };
    }

    if flag {
        println!("NaN");
    } else {
        println!("{}", total);
    }
}

fn read() -> (i128, bool) {
    let mut input = String::new();
    let bytes = io::stdin().read_line(&mut input).expect("Error!");

    if bytes == 0 {
        return (-1, true);
    }

    match input.trim().parse::<i128>() {
        Ok(num) => (num, false),
        Err(_) => (0, true),
    }
}
