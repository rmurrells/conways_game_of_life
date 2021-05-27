use conways_game_of_life_impl::{config, BResult, Grid, Grid1dVec};

fn main() -> BResult<()> {
    let mut grid = Grid1dVec::empty((80, 21));
    grid.set_fps(10);
    config::test(&mut grid)?;

    loop {
        print!("{esc}c", esc = 27 as char);
        println!("{}", grid);
        grid.update();
    }
}
