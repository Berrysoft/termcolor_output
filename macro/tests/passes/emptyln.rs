use termcolor::NoColor;
use termcolor_output as tco;

fn main() {
    let mut w: NoColor<Vec<u8>> = NoColor::new(vec![]);
    match tco::writeln!(w) {
        Ok(_) => {}
        Err(_) => {}
    };
}
