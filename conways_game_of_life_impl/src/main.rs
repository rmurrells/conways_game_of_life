use conways_game_of_life_impl::{config, BResult, Grid, Grid2d};

fn main() -> BResult<()> {
    let mut grid = Grid2d::empty((80, 21));
    config::test(&mut grid)?;

    for _ in 0..1000 {
        print!("{esc}c", esc = 27 as char);
        println!("{}", grid);
        grid.update();
    }
    Ok(())
}
