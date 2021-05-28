use conways_game_of_life_sdl::{
    config,
    renderer::{CyclicalModulatorOpt, DrawOption, Rygcbm},
    Grid2dVec, SDLInterfaceBuilder,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let size = (600, 600);
    let mut grid = Grid2dVec::empty(size);
    config::random(&mut grid, 0.25)?;

    let mut interface_builder = SDLInterfaceBuilder::new()?;
    interface_builder
        .renderer_builder
        .video_subsystem_command(move |mut vss| {
            vss.window_size = (size.0 as u32, size.1 as u32);
            vss
        });
    interface_builder.renderer_builder.draw_opt =
        DrawOption::DynamicCyclical(CyclicalModulatorOpt::Rygcbm(Rygcbm::Red));

    let mut interface = interface_builder.build(grid)?;
    interface.run()?;
    Ok(())
}
