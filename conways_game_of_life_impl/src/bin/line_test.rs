use conways_game_of_life_impl::{BResult, Grid, Grid2dArr};

fn main() -> BResult<()> {
    let mut grid = Grid2dArr::<80, 21>::empty();
    grid.set_fps(10);
    grid.set_line((0, 0), (5, 5), true)?;
    grid.set_line((2, 0), (20, 1), true)?;
    grid.set_line((0, 2), (1, 20), true)?;
    grid.set_line((5, 10), (20, 10), true)?;
    grid.set_line((10, 12), (10, 20), true)?;
    grid.set_line((10, 4), (10, 4), true)?;
    grid.set_line((10, 6), (11, 6), true)?;
    grid.set_line((10, 8), (12, 8), true)?;
    println!("{}", grid);
    Ok(())
}
