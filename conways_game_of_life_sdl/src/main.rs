use conways_game_of_life_sdl::{config, Grid2d, SDLInterfaceBuilder};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut grid = Grid2d::empty((80, 80));
    config::test(&mut grid)?;

    let mut interface_builder = SDLInterfaceBuilder::new()?;
    interface_builder
        .renderer_builder
        .video_subsystem_command(|vss| {
            vss.window_size = (600, 600);
        });

    let mut interface = interface_builder.build(grid)?;
    interface.run()?;
    Ok(())
}
