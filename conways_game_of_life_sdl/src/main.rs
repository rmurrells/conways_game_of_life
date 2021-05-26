use conways_game_of_life_sdl::{config, Grid, Grid2dVec, SDLInterfaceBuilder};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let size = (600, 600);
    let mut grid = Grid2dVec::empty(size);
    config::random(&mut grid, 0.3)?;
    grid.set_fps(0);

    let mut interface_builder = SDLInterfaceBuilder::new()?;
    interface_builder
        .renderer_builder
        .video_subsystem_command(move |mut vss| {
            vss.window_size = size;
            vss
        });

    let mut interface = interface_builder.build(grid)?;
    interface.run()?;
    Ok(())
}
