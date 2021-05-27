use conways_game_of_life_sdl::{
    config, Grid, Grid1dVec, Grid2dArr, Grid2dVec, GridUnit, SDLInterfaceBuilder,
};
use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion,
};
use std::time::Duration;

fn init_benchmark<G: Grid>(name: &str, mut grid: G, group: &mut BenchmarkGroup<'_, WallTime>) {
    grid.set_fps(0);
    config::random(&mut grid, 0.25).unwrap();

    let size = grid.size();
    let mut interface_builder = SDLInterfaceBuilder::new().unwrap();
    interface_builder
        .renderer_builder
        .video_subsystem_command(move |mut vss| {
            vss.window_size = (size.0 as u32, size.1 as u32);
            vss
        });

    let mut interface = interface_builder.build(grid).unwrap();
    group.bench_function(name, |b| {
        b.iter(|| {
            interface.tick().unwrap();
        })
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("random");
    group.measurement_time(Duration::from_secs(10));

    const WIDTH: usize = 600;
    const HEIGHT: usize = 600;
    let size = (WIDTH as GridUnit, HEIGHT as GridUnit);
    init_benchmark("1d_vec", Grid1dVec::empty(size), &mut group);
    init_benchmark("1d_vec_box", Box::new(Grid1dVec::empty(size)), &mut group);
    init_benchmark("2d_vec", Grid2dVec::empty(size), &mut group);
    init_benchmark("2d_vec_box", Box::new(Grid2dVec::empty(size)), &mut group);
    init_benchmark("2d_arr", Grid2dArr::<WIDTH, HEIGHT>::empty(), &mut group);
    init_benchmark(
        "2d_arr_box",
        Box::new(Grid2dArr::<WIDTH, HEIGHT>::empty()),
        &mut group,
    );

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
