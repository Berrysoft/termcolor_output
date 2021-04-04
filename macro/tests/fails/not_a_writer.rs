use termcolor_output as tco;

fn main() {
    let not_a_writer = 0u32;
    tco::write!(not_a_writer, "test");
}
