fn main() {
    let board = [['0'; 3]; 3];

    for row in 0..3 {
        for col in 0..3 {
            print!("{}", board[row][col]);
        }
        println!();
    }
}
