use criterion::{BenchmarkGroup, criterion_group, criterion_main, Criterion, measurement::WallTime};
use conways_game_of_life_impl::{config, Grid, Grid2d, Grid2dArr, GridUnit, LinearGrid};
use std::time::Duration;

fn init_benchmark<G: Grid>(name: &str, mut grid: G, group: &mut BenchmarkGroup<'_, WallTime>) {
    grid.set_fps(0);
    config::test(&mut grid).unwrap();
    
    group.bench_function(name, |b| {
        b.iter(|| {
	    grid.update();
        })
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("bgroup");
    group.measurement_time(Duration::from_secs(10));

    const WIDTH: usize = 600;
    const HEIGHT: usize = 600;
    let size = (WIDTH as GridUnit, HEIGHT as GridUnit);
    init_benchmark("linear", LinearGrid::empty(size), &mut group);
    init_benchmark("2d", Grid2d::empty(size), &mut group);
    init_benchmark("2d_arr", Grid2dArr::<WIDTH, HEIGHT>::empty(), &mut group);

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
