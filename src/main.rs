use crate::components::mine_sweeper::MineSweeper;

mod components;

fn main() {
	wasm_logger::init(wasm_logger::Config::default());
	yew::start_app::<MineSweeper>();
}
