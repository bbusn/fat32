pub mod consts;
pub mod helpers;

pub use helpers::*;
pub use consts::*;

pub fn init_cli() {
    clear_cli();
    print_ascii();
}
