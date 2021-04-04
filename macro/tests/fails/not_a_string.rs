use termcolor_output as tco;

fn main() {
    let writer: Vec<u8> = vec![];
    let not_a_string = 0u32;
    tco::write!(&mut writer, not_a_string);
}
