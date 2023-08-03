use clap::Parser;
use spinoza::{
    config::{Config, QSArgs},
    core::{State, CONFIG},
    gates::{apply, Gate},
    utils::{pretty_print_int, to_table},
};

fn rz(n: usize, show_results: bool) {
    let now = std::time::Instant::now();
    let mut state = State::new(n);

    for t in 0..n {
        apply(Gate::RZ(1.0), &mut state, t);
    }

    let elapsed = now.elapsed().as_micros();
    println!("{}", pretty_print_int(elapsed));
    if show_results {
        to_table(&state);
    }
}

fn main() {
    let args = QSArgs::parse();
    let config = Config::from_cli(args);
    CONFIG.set(config).unwrap();
    rz(config.qubits.into(), config.print);
}
