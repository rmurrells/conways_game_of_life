use conways_game_of_life_sdl::{config, Grid2d, SDLInterface};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut grid = Grid2d::empty((80, 21));
    config::test(&mut grid)?;
    let mut interface = SDLInterface::new("conways_game_of_life", (800, 600), grid)?;
    while interface.tick()? {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    Ok(())
}
