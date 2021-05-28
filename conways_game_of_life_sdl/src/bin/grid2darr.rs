use conways_game_of_life_sdl::{
    config,
    renderer::{CyclicalModulatorOpt, DrawOption, Rygcbm},
    Grid2dArr, SDLInterfaceBuilder,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    const WIDTH: usize = 600;
    const HEIGHT: usize = 600;
    let mut grid = Grid2dArr<WIDTH, HEIGHR>::empty();
    config::random(&mut grid, 0.25)?;

    let mut interface_builder = SDLInterfaceBuilder::new()?;
    interface_builder
        .renderer_builder
        .video_subsystem_command(move |mut vss| {
            vss.window_size = (WIDTH as u32, HEIGHT as u32);
            vss
        });
    interface_builder.renderer_builder.draw_opt =
        DrawOption::DynamicCyclical(CyclicalModulatorOpt::Rygcbm(Rygcbm::Red));

    let mut interface = interface_builder.build(grid)?;
    interface.run()?;
    Ok(())
}
