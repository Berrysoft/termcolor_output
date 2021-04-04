use termcolor::NoColor;
use termcolor_output as tco;

fn main() {
    let mut w: NoColor<Vec<u8>> = NoColor::new(vec![]);
    match tco::reset_guard(&mut w, |w| tco::writeln!(w, "Hello world!")) {
        Ok(_) => {}
        Err(_) => {}
    };
}
