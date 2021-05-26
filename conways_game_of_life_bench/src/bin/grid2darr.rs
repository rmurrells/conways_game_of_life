use conways_game_of_life_impl::{config, BResult, Grid, Grid2dArr};

fn main() -> BResult<()> {
    let mut grid = Grid2dArr::<600, 600>::empty();
    grid.set_fps(0);
    config::test(&mut grid)?;
    loop {
	grid.update();
    }
}
