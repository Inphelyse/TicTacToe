use std::io;

fn main() {
    let mut board = [['0'; 3]; 2];

    for row in 0..2 {
        for col in 0..3 {
            println!("Enter a number for [{row}][{col}]");
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            board[row][col] = input.trim().parse().unwrap();
        }
    }

    println!("Result");
    for row in 0..2 {
        for col in 0..3 {
            print!("{}", board[row][col]);
        }
        println!("");
    }
}
