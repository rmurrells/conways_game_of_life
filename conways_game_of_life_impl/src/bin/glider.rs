use conways_game_of_life_impl::{config, BResult, Grid, Grid2dArr};

fn main() -> BResult<()> {
    let mut grid = Grid2dArr::<80, 20>::empty();
    grid.set_fps(10);
    for x in 0..=15 {
        for y in 0..=3 {
            config::glider(&mut grid, (x * 5, y * 5))?;
        }
    }

    loop {
        print!("{esc}c", esc = 27 as char);
        println!("{}", grid);
        grid.update();
    }
}
