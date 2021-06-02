use conways_game_of_life_impl::{config, BResult, Grid, Grid2dArr};

fn main() -> BResult<()> {
    let mut grid = Grid2dArr::<80, 21>::empty();
    grid.set_fps(10);
    config::test(&mut grid)?;

    loop {
        print!("{esc}c", esc = 27 as char);
        println!("{}", grid);
        grid.update();
    }
}
