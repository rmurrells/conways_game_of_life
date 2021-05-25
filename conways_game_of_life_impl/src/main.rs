use conways_game_of_life_impl::{config, BResult, Grid, Grid2d};
use std::{thread, time::Duration};

fn main() -> BResult<()> {
    let mut grid = Grid2d::empty((80, 21));
    config::test(&mut grid)?;

    for _ in 0..1000 {
        print!("{esc}c", esc = 27 as char);
        println!("{}", grid);
        grid.update();
        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
