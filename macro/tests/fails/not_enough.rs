use termcolor_output as tco;

fn main() {
    let writer: Vec<u8> = vec![];
    tco::write!(
        writer,
        "Before first - {}, then - {:?}, or even {{this}}: {:b}!"
    );
}
