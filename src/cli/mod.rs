pub mod consts;
pub mod helpers;

pub use consts::*;
pub use helpers::*;

pub fn init_cli() {
    clear_cli();
    print_ascii();
}
