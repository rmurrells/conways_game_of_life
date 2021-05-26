use conways_game_of_life_impl::{config, BResult, Grid, Grid2dVec};

fn main() -> BResult<()> {
    let mut grid = Grid2dVec::empty((600, 600));
    grid.set_fps(0);
    config::random(&mut grid, 0.25)?;
    loop {
        let now = std::time::Instant::now();
        grid.update();
        println!("{:?}", now.elapsed());
    }
}
