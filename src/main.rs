mod playground_1;
pub use crate::playground_1::main_1_2;

mod playground_2;
pub use crate::playground_2::main_2_2;
pub use crate::playground_2::main_2_3;

fn main() {
    main_1_2::main_1_2();

    main_2_2::main_2_2();
    main_2_3::main_2_3();
}
