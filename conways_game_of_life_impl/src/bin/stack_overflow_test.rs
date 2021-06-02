use conways_game_of_life_impl::Grid2dArr;

fn main() {
    let grid = Grid2dArr::<2000, 2000>::empty();
    let _grid = grid.clone();
    drop(grid);
}
