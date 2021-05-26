use criterion::{BenchmarkGroup, criterion_group, criterion_main, Criterion, measurement::WallTime};
use conways_game_of_life_impl::{config, Grid, Grid2d, LinearGrid};
use std::time::Duration;

fn init_benchmark<G: Grid>(name: &str, mut grid: G, group: &mut BenchmarkGroup<'_, WallTime>) {
    grid.set_fps(0);
    
    config::block(&mut grid, (1, 1)).unwrap();
    config::bee_hive(&mut grid, (5, 1)).unwrap();
    config::loaf(&mut grid, (11, 1)).unwrap();
    config::boat(&mut grid, (17, 1)).unwrap();
    config::tub(&mut grid, (22, 1)).unwrap();
    config::blinker(&mut grid, (28, 1)).unwrap();
    config::toad(&mut grid, (32, 1)).unwrap();
    config::beacon(&mut grid, (38, 1)).unwrap();
    config::pulsar(&mut grid, (45, 1)).unwrap();
    config::penta_decathlon(&mut grid, (64, 3)).unwrap();
    
    group.bench_function(name, |b| {
        b.iter(|| {
	    grid.update();
        })
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("bgroup");
    group.measurement_time(Duration::from_secs(10));

    let size = (600, 600);
    init_benchmark("linear", LinearGrid::empty(size), &mut group);
    init_benchmark("2d", Grid2d::empty(size), &mut group);

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
