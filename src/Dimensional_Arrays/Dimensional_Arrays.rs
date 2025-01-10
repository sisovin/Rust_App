// 2 Dimensional Arrays

fn main() {
    // Define a 2-dimensional array
    let mut number_grid = vec![vec![0; 2]; 2];
    // number_grid = [ [1, 2], [3, 4] ]
    number_grid[0][0] = 99;

    println!("{}", number_grid[0][0]);
    println!("{}", number_grid[0][1]);
}
